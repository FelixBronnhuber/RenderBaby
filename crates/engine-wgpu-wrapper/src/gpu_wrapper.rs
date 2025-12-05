use crate::bind_group;
use crate::{GpuDevice, buffers, pipeline};
use anyhow::{Ok, Result, anyhow};
use bind_group::{BindGroup, BindGroupLayout};
use buffers::GpuBuffers;
use engine_config::render_config::{Change, Validate, ValidateInit};
use engine_config::{RenderConfig, Uniforms};
use pipeline::ComputePipeline;

const TOTAL_SAMPLES: u32 = 5000; //has to be multiple of SAMPLES_PER_PASS
const SAMPLES_PER_PASS: u32 = 10; //also change in shader!

pub struct GpuWrapper {
    buffer_wrapper: GpuBuffers,
    bind_group_wrapper: BindGroup,
    bind_group_layout_wrapper: BindGroupLayout,
    device: wgpu::Device,
    queue: wgpu::Queue,
    rc: RenderConfig,
    uniforms: Uniforms,
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
        let initial_uniforms = match rc.uniforms {
            Change::Create(u) | Change::Update(u) => u,
            Change::Keep => Uniforms::default(),
            Change::Delete => panic!("Cannot create GpuWrapper with deleted uniforms"),
        };
        Ok(Self {
            buffer_wrapper: buffers,
            bind_group_layout_wrapper: layout,
            bind_group_wrapper: groups,
            device: gpu.device,
            queue: gpu.queue,
            rc,
            uniforms: initial_uniforms,
            pipeline_wrapper: pipeline,
            initialized: false,
        })
    }

    pub fn update(&mut self, new_rc: RenderConfig) -> Result<()> {
        if !self.initialized {
            // First render: require Create for all fields
            new_rc.validate_init()?;
            // Initialize all resources
            if let Change::Create(uniforms) = &new_rc.uniforms {
                // Check if resolution changed from engine initialization and resize buffers
                let old_size = self.get_image_buffer_size() * 4;
                let new_size = (uniforms.width as u64) * (uniforms.height as u64) * 4;
                self.buffer_wrapper.init_uniforms(&self.device, uniforms);
                if old_size != new_size {
                    log::info!(
                        "Resolution changed during first render, resizing buffers from {} to {} bytes",
                        old_size,
                        new_size
                    );
                    self.uniforms = *uniforms;
                    self.buffer_wrapper.grow_resolution(&self.device, new_size);
                }
            }
            if let Change::Create(spheres) = &new_rc.spheres {
                self.buffer_wrapper.init_spheres(&self.device, spheres);
            }
            if let Change::Create(vertices) = &new_rc.vertices {
                self.buffer_wrapper.init_vertices(&self.device, vertices);
            }
            if let Change::Create(triangles) = &new_rc.triangles {
                self.buffer_wrapper.init_triangles(&self.device, triangles);
            }
            // Recreate bind group with new buffers
            self.recreate_bind_group();
            self.initialized = true;
        } else {
            // Subsequent updates: validate and apply changes
            new_rc.validate()?;

            match &new_rc.uniforms {
                Change::Keep => log::info!("Not updating Uniforms."),
                Change::Update(uniforms) => {
                    // Check if resolution changed and resize output/staging buffers if needed
                    let old_size = self.get_image_buffer_size() * 4;
                    let new_size = (uniforms.width as u64) * (uniforms.height as u64) * 4;
                    if old_size != new_size {
                        log::info!(
                            "Resolution changed, resizing buffers from {} to {} bytes",
                            old_size,
                            new_size
                        );
                        self.buffer_wrapper.grow_resolution(&self.device, new_size);
                    }
                    self.uniforms = *uniforms;
                    self.buffer_wrapper.update_uniforms(&self.device, uniforms);
                }
                Change::Delete => {
                    self.buffer_wrapper.delete_uniforms(&self.device);
                }
                Change::Create(_) => {
                    log::warn!("Create not allowed after initialization for uniforms.");
                }
            }

            match &new_rc.spheres {
                Change::Keep => log::info!("Not updating Spheres Buffer."),
                Change::Update(spheres) => {
                    self.buffer_wrapper.update_spheres(&self.device, spheres);
                }
                Change::Delete => {
                    self.buffer_wrapper.delete_spheres(&self.device);
                }
                Change::Create(_) => {
                    log::warn!("Create not allowed after initialization for spheres.");
                }
            }

            match &new_rc.vertices {
                Change::Keep => log::info!("Not updating Vertices Buffer."),
                Change::Update(vertices) => {
                    self.buffer_wrapper.update_vertices(&self.device, vertices);
                }
                Change::Delete => {
                    self.buffer_wrapper.delete_vertices(&self.device);
                }
                Change::Create(_) => {
                    log::warn!("Create not allowed after initialization for vertices.");
                }
            }

            match &new_rc.triangles {
                Change::Keep => log::info!("Not updating Triangles Buffer."),
                Change::Update(triangles) => {
                    self.buffer_wrapper
                        .update_triangles(&self.device, triangles);
                }
                Change::Delete => {
                    self.buffer_wrapper.delete_triangles(&self.device);
                }
                Change::Create(_) => {
                    log::warn!("Create not allowed after initialization for triangles.");
                }
            }
            // Recreate bind group after any buffer updates
            self.recreate_bind_group();
        }

        self.rc = new_rc;
        Ok(())
    }

    fn recreate_bind_group(&mut self) {
        self.bind_group_wrapper = BindGroup::new(
            &self.device,
            &self.buffer_wrapper,
            &self.bind_group_layout_wrapper.bind_group_layout,
        );
    }

    pub fn get_image_buffer_size(&self) -> u64 {
        let uniforms = match &self.rc.uniforms {
            Change::Create(u) | Change::Update(u) => u,
            Change::Keep | Change::Delete => panic!("Uniforms must be initialized"),
        };
        (uniforms.width as u64) * (uniforms.height as u64)
    }

    pub fn get_width(&self) -> u32 {
        match &self.rc.uniforms {
            Change::Create(u) | Change::Update(u) => u.width,
            Change::Keep | Change::Delete => panic!("Uniforms must be initialized"),
        }
    }

    pub fn get_height(&self) -> u32 {
        match &self.rc.uniforms {
            Change::Create(u) | Change::Update(u) => u.height,
            Change::Keep | Change::Delete => panic!("Uniforms must be initialized"),
        }
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

    pub fn dispatch_compute_progressive(&self, pass_index: u32, total_passes: u32) -> Result<()> {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some(&format!("Compute Pass {}/{}", pass_index + 1, total_passes)),
            });

        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute Pass"),
                timestamp_writes: None,
            });

            pass.set_pipeline(self.get_pipeline());
            pass.set_bind_group(0, self.get_bind_group(), &[]);
            pass.dispatch_workgroups(
                self.get_width().div_ceil(16),
                self.get_height().div_ceil(16),
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

        let _ = self.device.poll(wgpu::PollType::wait_indefinitely());

        Ok(())
    }

    pub fn dispatch_compute(&mut self) -> Result<()> {
        self.queue.write_buffer(
            &self.buffer_wrapper.accumulation,
            0,
            &vec![0u8; (self.get_width() * self.get_height() * 16) as usize],
        );
        let total_passes = TOTAL_SAMPLES.div_ceil(SAMPLES_PER_PASS);

        for pass in 0..total_passes {
            println!("Rendering pass {}/{}", pass + 1, total_passes);
            let mut uniforms = self.uniforms;
            uniforms.current_pass = pass;
            self.queue.write_buffer(
                &self.buffer_wrapper.uniforms,
                0,
                bytemuck::cast_slice(&[uniforms]),
            );

            self.dispatch_compute_progressive(pass, total_passes)?;
        }

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
        let mut uniforms = match &self.rc.uniforms {
            Change::Create(u) | Change::Update(u) => *u,
            Change::Keep | Change::Delete => panic!("Uniforms must be initialized"),
        };

        let spheres_len = match &self.rc.spheres {
            Change::Create(s) | Change::Update(s) => s.len(),
            Change::Keep | Change::Delete => 0,
        };
        let triangles_len = match &self.rc.triangles {
            Change::Create(t) | Change::Update(t) => t.len(),
            Change::Keep | Change::Delete => 0,
        };

        uniforms.spheres_count = spheres_len as u32;
        uniforms.triangles_count = triangles_len as u32 / 3;

        log::info!(
            "Writing uniforms to GPU: camera_pos={:?}, camera_dir={:?}, pane_distance={}, pane_width={}, size={}x{}, spheres={}, triangles={}",
            uniforms.camera.pos,
            uniforms.camera.dir,
            uniforms.camera.pane_distance,
            uniforms.camera.pane_width,
            uniforms.width,
            uniforms.height,
            uniforms.spheres_count,
            uniforms.triangles_count
        );

        self.queue.write_buffer(
            &self.buffer_wrapper.uniforms,
            0,
            bytemuck::bytes_of(&uniforms),
        );

        if let Change::Create(spheres) | Change::Update(spheres) = &self.rc.spheres {
            self.queue.write_buffer(
                &self.buffer_wrapper.spheres,
                0,
                bytemuck::cast_slice(spheres),
            );
        }

        if let Change::Create(vertices) | Change::Update(vertices) = &self.rc.vertices {
            self.queue.write_buffer(
                &self.buffer_wrapper.vertices,
                0,
                bytemuck::cast_slice(vertices),
            );
        }

        if let Change::Create(triangles) | Change::Update(triangles) = &self.rc.triangles {
            self.queue.write_buffer(
                &self.buffer_wrapper.triangles,
                0,
                bytemuck::cast_slice(triangles),
            );
        }
    }
}
