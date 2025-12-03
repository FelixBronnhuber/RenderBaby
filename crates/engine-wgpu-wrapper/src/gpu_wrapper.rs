use crate::bind_group;
use crate::{GpuDevice, buffers, pipeline};
use anyhow::{Result, anyhow};
use bind_group::{BindGroup, BindGroupLayout};
use buffers::GpuBuffers;
use engine_config::render_config::{Update, Validate, ValidateInit};
use engine_config::{RenderConfig, RenderConfigBuilderError, Sphere};
use pipeline::ComputePipeline;

const RGBA_4: u64 = 4;
const DISPATCH_WORK_GROUP_WIDTH: u32 = 16;
const DISPATCH_WORK_GROUP_HEIGHT: u32 = 16;

pub struct GpuWrapper {
    buffer_wrapper: GpuBuffers,
    bind_group_wrapper: BindGroup,
    bind_group_layout_wrapper: BindGroupLayout,
    device: wgpu::Device,
    queue: wgpu::Queue,
    rc: RenderConfig,
    pipeline_wrapper: ComputePipeline,
    initialized: bool,
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
            initialized: false,
        })
    }

    pub fn update(&mut self, new_rc: RenderConfig) -> Result<()> {
        if !self.initialized {
            // First render: require Create for all fields
            new_rc.validate_init()?;
            // Initialize all resources
            if let Update::Create(uniforms) = &new_rc.uniforms {
                // Initialize uniforms buffer
                self.buffer_wrapper.init_uniforms(&self.device, uniforms);
            }
            if let Update::Create(spheres) = &new_rc.spheres {
                self.buffer_wrapper.init_spheres(&self.device, spheres);
            }
            if let Update::Create(vertices) = &new_rc.vertices {
                self.buffer_wrapper.init_vertices(&self.device, vertices);
            }
            if let Update::Create(triangles) = &new_rc.triangles {
                self.buffer_wrapper.init_triangles(&self.device, triangles);
            }
            self.initialized = true;
            self.rc = new_rc;
            return Ok(());
        }

        // Subsequent updates: validate and apply changes
        new_rc.validate()?;

        match &new_rc.uniforms {
            Update::Keep => log::info!("Not updating Uniforms."),
            Update::Update(uniforms) => {
                self.buffer_wrapper.update_uniforms(&self.device, uniforms);
            }
            Update::Delete => {
                self.buffer_wrapper.delete_uniforms(&self.device);
            }
            Update::Create(_) => {
                log::warn!("Create not allowed after initialization for uniforms.");
            }
        }

        match &new_rc.spheres {
            Update::Keep => log::info!("Not updating Spheres Buffer."),
            Update::Update(spheres) => {
                self.buffer_wrapper.update_spheres(&self.device, spheres);
            }
            Update::Delete => {
                self.buffer_wrapper.delete_spheres(&self.device);
            }
            Update::Create(_) => {
                log::warn!("Create not allowed after initialization for spheres.");
            }
        }

        match &new_rc.vertices {
            Update::Keep => log::info!("Not updating Vertices Buffer."),
            Update::Update(vertices) => {
                self.buffer_wrapper.update_vertices(&self.device, vertices);
            }
            Update::Delete => {
                self.buffer_wrapper.delete_vertices(&self.device);
            }
            Update::Create(_) => {
                log::warn!("Create not allowed after initialization for vertices.");
            }
        }

        match &new_rc.triangles {
            Update::Keep => log::info!("Not updating Triangles Buffer."),
            Update::Update(triangles) => {
                self.buffer_wrapper
                    .update_triangles(&self.device, triangles);
            }
            Update::Delete => {
                self.buffer_wrapper.delete_triangles(&self.device);
            }
            Update::Create(_) => {
                log::warn!("Create not allowed after initialization for triangles.");
            }
        }

        self.rc = new_rc;
        Ok(())
    }

    pub fn get_image_buffer_size(&self) -> u64 {
        (self.rc.uniforms.width as u64) * (self.rc.uniforms.height as u64)
    }

    pub fn get_width(&self) -> u32 {
        self.rc.uniforms.width
    }

    pub fn get_height(&self) -> u32 {
        self.rc.uniforms.height
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
                self.get_width().div_ceil(DISPATCH_WORK_GROUP_WIDTH),
                self.get_height().div_ceil(DISPATCH_WORK_GROUP_HEIGHT),
                1,
            );
        }

        encoder.copy_buffer_to_buffer(
            &self.buffer_wrapper.output,
            0,
            &self.buffer_wrapper.staging,
            0,
            self.get_image_buffer_size() * 4,
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
        let mut result = Vec::with_capacity((self.get_image_buffer_size() * 4) as usize);

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
        let mut uniforms = self.rc.uniforms;
        uniforms.spheres_count = self.rc.spheres.len() as u32;
        uniforms.triangles_count = self.rc.triangles.len() as u32 / 3;

        self.queue.write_buffer(
            &self.buffer_wrapper.uniforms,
            0,
            bytemuck::bytes_of(&uniforms),
        );

        let spheres: Vec<Sphere> = self.rc.spheres.clone();
        self.queue.write_buffer(
            &self.buffer_wrapper.spheres,
            0,
            bytemuck::cast_slice(&spheres),
        );

        let vertices = &self.rc.vertices;
        self.queue.write_buffer(
            &self.buffer_wrapper.vertices,
            0,
            bytemuck::cast_slice(vertices),
        );

        let triangles = &self.rc.triangles;
        self.queue.write_buffer(
            &self.buffer_wrapper.triangles,
            0,
            bytemuck::cast_slice(triangles),
        );
    }
}
