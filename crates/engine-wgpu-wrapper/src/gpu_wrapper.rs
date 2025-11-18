use crate::bind_group;
use crate::{GpuDevice, buffers, pipeline};
use anyhow::{Result, anyhow};
use bind_group::{BindGroup, BindGroupLayout};
use buffers::GpuBuffers;
use engine_config::{RenderConfig, Sphere};
use pipeline::ComputePipeline;

pub struct GpuWrapper {
    buffer_wrapper: GpuBuffers,
    bind_group_wrapper: BindGroup,
    bind_group_layout_wrapper: BindGroupLayout,
    device: wgpu::Device,
    queue: wgpu::Queue,
    rc: RenderConfig,
    pipeline_wrapper: ComputePipeline,
}

impl GpuWrapper {
    ///initializes shared Config, deligated to Sub modules
    pub fn new(rc: RenderConfig, path: &str) -> Result<Self> {
        let gpu = GpuDevice::new().unwrap();
        let buffers = GpuBuffers::new(&rc, &gpu.device);
        let layout = BindGroupLayout::new(&gpu.device);
        let groups = BindGroup::new(&gpu.device, &buffers, &layout.bind_group_layout);
        let pipeline = ComputePipeline::new(&gpu.device, &layout.bind_group_layout, path);
        Ok(Self {
            buffer_wrapper: buffers,
            bind_group_layout_wrapper: layout,
            bind_group_wrapper: groups,
            device: gpu.device,
            queue: gpu.queue,
            rc,
            pipeline_wrapper: pipeline,
        })
    }

    pub fn update(&mut self, rc: RenderConfig) {
        let new_size = (rc.camera.height as u64) * (rc.camera.width as u64);
        if self.get_size() != new_size {
            self.buffer_wrapper
                .grow_resolution(&self.device, new_size * 4);
            self.bind_group_wrapper = BindGroup::new(
                &self.device,
                &self.buffer_wrapper,
                self.get_bind_group_layout(),
            )
        }
        self.rc = rc;
        //todo!("Also grow the size of the other buffers");
    }

    pub fn get_size(&self) -> u64 {
        (self.rc.camera.width as u64) * (self.rc.camera.height as u64)
    }

    pub fn get_width(&self) -> u32 {
        self.rc.camera.width
    }

    pub fn get_height(&self) -> u32 {
        self.rc.camera.height
    }

    pub fn get_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout_wrapper.bind_group_layout
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group_wrapper.bind_group
    }

    pub fn get_pipeline(&self) -> &wgpu::ComputePipeline {
        &self.pipeline_wrapper.pipeline
    }

    pub fn dispatch_compute(&self) -> Result<()> {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute Pass"),
                timestamp_writes: None,
            });

            pass.set_pipeline(self.get_pipeline());
            pass.set_bind_group(0, self.get_bind_group(), &[]);
            pass.dispatch_workgroups(
                self.get_width().div_ceil(8),
                self.get_height().div_ceil(8),
                1,
            );
        }

        encoder.copy_buffer_to_buffer(
            &self.buffer_wrapper.output,
            0,
            &self.buffer_wrapper.staging,
            0,
            self.get_size() * 4,
        );

        self.queue.submit(Some(encoder.finish()));
        Ok(())
    }

    pub fn read_pixels(&self) -> Result<Vec<u8>> {
        let buffer_slice = self.buffer_wrapper.staging.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();

        buffer_slice.map_async(wgpu::MapMode::Read, move |res| {
            let _ = sender.send(res);
        });

        self.device
            .poll(wgpu::PollType::wait_indefinitely())
            .map_err(|e| anyhow!("Device poll failed: {:?}", e))?;

        receiver
            .recv()
            .map_err(|_| anyhow!("Failed to receive map_async result"))??;

        let data_slice = buffer_slice.get_mapped_range();
        let mut result = Vec::with_capacity((self.get_size() * 4) as usize);

        for chunk in data_slice.chunks_exact(4) {
            result.push(chunk[0]);
            result.push(chunk[1]);
            result.push(chunk[2]);
            result.push(255u8);
        }

        drop(data_slice);
        self.buffer_wrapper.staging.unmap();
        Ok(result)
    }

    pub fn update_uniforms(&self) {
        let camera = &self.rc.camera;
        self.queue
            .write_buffer(&self.buffer_wrapper.camera, 0, bytemuck::bytes_of(camera));

        let spheres: Vec<Sphere> = self.rc.spheres.clone();
        self.queue.write_buffer(
            &self.buffer_wrapper.spheres,
            0,
            bytemuck::cast_slice(&spheres),
        );

        let verticies = &self.rc.verticies;
        self.queue.write_buffer(
            &self.buffer_wrapper.verticies,
            0,
            bytemuck::cast_slice(verticies),
        );

        let triangles = &self.rc.triangles;
        self.queue.write_buffer(
            &self.buffer_wrapper.triangles,
            0,
            bytemuck::cast_slice(triangles),
        );
    }
}
