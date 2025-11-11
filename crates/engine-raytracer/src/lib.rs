use anyhow::{Result, anyhow};
pub use engine_config::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct RenderOutput {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>, // RGBA8 data
}

impl RenderOutput {
    pub fn new(width: usize, height: usize, pixels: Vec<u8>) -> Self {
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn expected_size(&self) -> usize {
        self.width * self.height * 4
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        let expected = self.expected_size();
        if self.pixels.len() != expected {
            anyhow::bail!(
                "RenderOutput pixel size mismatch: expected {} bytes, got {}",
                expected,
                self.pixels.len()
            );
        }
        Ok(())
    }
}

pub struct ComputePipelineResources {
    pub pipeline: wgpu::ComputePipeline,
}

impl ComputePipelineResources {
    pub fn new(device: &wgpu::Device, bind_group_layout: &wgpu::BindGroupLayout) -> Self {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[bind_group_layout],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("main"),
            cache: None,
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        });

        Self { pipeline }
    }
}

pub struct RenderState {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    pipeline: wgpu::ComputePipeline,
    output_buffer: wgpu::Buffer,
    staging_buffer: wgpu::Buffer,
    main_bind_group: wgpu::BindGroup,
    dimensions_buffer: wgpu::Buffer,
    spheres_buffer: wgpu::Buffer,
    width: usize,
    height: usize,
}

impl RenderState {
    pub fn new(
        device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
        output_buffer: wgpu::Buffer,
        staging_buffer: wgpu::Buffer,
        bind_group_layout: wgpu::BindGroupLayout,
        bind_group: wgpu::BindGroup,
        dimensions_buffer: wgpu::Buffer,
        spheres_buffer: wgpu::Buffer,
        dimensions: (usize, usize),
    ) -> Self {
        let pipeline = ComputePipelineResources::new(&device, &bind_group_layout).pipeline;

        Self {
            device,
            queue,
            pipeline,
            output_buffer,
            staging_buffer,
            main_bind_group: bind_group,
            dimensions_buffer,
            spheres_buffer,
            width: dimensions.0,
            height: dimensions.1,
        }
    }

    fn update_from(&mut self, rc: RenderCommand) {
        // Update camera (dimensions/fov)
        let camera = Camera {
            width: self.width as u32,
            height: self.height as u32,
            fov: rc.fov.unwrap_or(std::f32::consts::FRAC_PI_4),
        };

        self.queue
            .write_buffer(&self.dimensions_buffer, 0, bytemuck::bytes_of(&camera));

        // Update spheres
        let spheres: Vec<Sphere> = rc.spheres.clone();

        self.queue
            .write_buffer(&self.spheres_buffer, 0, bytemuck::cast_slice(&spheres));
    }

    pub fn render(&mut self, rc: RenderCommand) -> Result<RenderOutput> {
        self.dispatch_compute()?;

        self.update_from(rc);

        let buffer_slice = self.staging_buffer.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();

        buffer_slice.map_async(wgpu::MapMode::Read, move |res| {
            let _ = sender.send(res);
        });

        self.device
            .poll(wgpu::PollType::wait_indefinitely())
            .map_err(|e| anyhow!("Device poll failed: {:?}", e))?;

        let map_result = receiver
            .recv()
            .map_err(|_| anyhow!("Failed to receive map_async result"))?;

        map_result.map_err(|_| anyhow!("GPU buffer mapping failed"))?;

        let data_slice = buffer_slice.get_mapped_range();
        let mut result = Vec::with_capacity(self.width * self.height * 4);

        for chunk in data_slice.chunks_exact(4) {
            result.extend_from_slice(&[chunk[0], chunk[1], chunk[2], 255]);
        }

        drop(data_slice);

        self.staging_buffer.unmap();

        if result.len() != self.width * self.height * 4 {
            return Err(anyhow!(
                "Render produced {} bytes but expected {}",
                result.len(),
                self.width * self.height * 4
            ));
        }

        Ok(RenderOutput {
            width: self.width,
            height: self.height,
            pixels: result,
        })
    }

    fn dispatch_compute(&self) -> Result<()> {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute Pass"),
                timestamp_writes: None,
            });

            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.main_bind_group, &[]);
            pass.dispatch_workgroups(
                (self.width as f32 / 8.0).ceil() as u32,
                (self.height as f32 / 8.0).ceil() as u32,
                1,
            );
        }

        encoder.copy_buffer_to_buffer(
            &self.output_buffer,
            0,
            &self.staging_buffer,
            0,
            (self.width * self.height * 4) as u64,
        );

        self.queue.submit(Some(encoder.finish()));
        Ok(())
    }
}
