use anyhow::{Result, anyhow};
pub use engine_config::{RenderConfig, Sphere};
use engine_wgpu_wrapper::GpuWrapper;

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
    pipeline: wgpu::ComputePipeline,
    gpu_wrapper: GpuWrapper
}

impl RenderState {
    #![allow(clippy::too_many_arguments)]
    pub fn new(rc:RenderConfig) -> Self {
        let camera = rc.camera.clone();
        let wrapper = GpuWrapper::new(rc);
        let pipeline = ComputePipelineResources::new(&wrapper.device, &wrapper.bind_group_layout.bind_group_layout).pipeline;

        Self {
            pipeline,
            gpu_wrapper: wrapper
        }
    }

    fn update_from(&mut self) {
        // Update camera (dimensions/fov)
        let camera = self.gpu_wrapper.rc.camera;

        self.gpu_wrapper.queue
            .write_buffer(&self.gpu_wrapper.buffers.camera, 0, bytemuck::bytes_of(&camera));

        // Update spheres
        let spheres: Vec<Sphere> = self.gpu_wrapper.rc.spheres.clone();

        self.gpu_wrapper.queue
            .write_buffer(&self.gpu_wrapper.buffers.spheres, 0, bytemuck::cast_slice(&spheres));
    }

    pub fn render(&mut self, rc: RenderConfig) -> Result<RenderOutput> {
        self.gpu_wrapper.update(rc);
        self.dispatch_compute()?;

        self.update_from();

        let buffer_slice = self.gpu_wrapper.buffers.staging.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();

        buffer_slice.map_async(wgpu::MapMode::Read, move |res| {
            let _ = sender.send(res);
        });

        self.gpu_wrapper.device
            .poll(wgpu::PollType::wait_indefinitely())
            .map_err(|e| anyhow!("Device poll failed: {:?}", e))?;

        let map_result = receiver
            .recv()
            .map_err(|_| anyhow!("Failed to receive map_async result"))?;

        map_result.map_err(|_| anyhow!("GPU buffer mapping failed"))?;

        let data_slice = buffer_slice.get_mapped_range();
        let mut result = Vec::with_capacity((self.gpu_wrapper.get_size() * 4) as usize);

        for chunk in data_slice.chunks_exact(4) {
            result.extend_from_slice(&[chunk[0], chunk[1], chunk[2], 255]);
        }

        drop(data_slice);

        self.gpu_wrapper.buffers.staging.unmap();

        if result.len() != (self.gpu_wrapper.get_size() * 4) as usize {
            return Err(anyhow!(
                "Render produced {} bytes but expected {}",
                result.len(),
                self.gpu_wrapper.get_size() * 4
            ));
        }

        Ok(RenderOutput {
            width: self.gpu_wrapper.get_width() as usize,
            height: self.gpu_wrapper.get_height() as usize,
            pixels: result,
        })
    }

    fn dispatch_compute(&self) -> Result<()> {
        let mut encoder = self
            .gpu_wrapper.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute Pass"),
                timestamp_writes: None,
            });

            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, self.gpu_wrapper.get_bind_group(), &[]);
            pass.dispatch_workgroups(
                (self.gpu_wrapper.get_width() as f32 / 8.0).ceil() as u32,
                (self.gpu_wrapper.get_height() as f32 / 8.0).ceil() as u32,
                1,
            );
        }

        encoder.copy_buffer_to_buffer(
            &self.gpu_wrapper.buffers.output,
            0,
            &self.gpu_wrapper.buffers.staging,
            0,
            (self.gpu_wrapper.get_size() * 4) as u64,
        );

        self.gpu_wrapper.queue.submit(Some(encoder.finish()));
        Ok(())
    }
}
