use crate::bind_group;
use crate::{GpuDevice, buffers, pipeline};
use anyhow::{Ok, Result, anyhow};
use bind_group::{BindGroup, BindGroupLayout};
use buffers::GpuBuffers;
use bytemuck::{Pod, Zeroable};
use engine_config::render_config::{Change, Validate, ValidateInit};
use engine_config::{RenderConfig, Uniforms};
use log::info;
use pipeline::ComputePipeline;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ProgressiveRenderHelper {
    pub total_passes: u32,
    pub current_pass: u32,
    pub total_samples: u32,
    pub samples_per_pass: u32,
}

impl ProgressiveRenderHelper {
    pub fn new(total_samples: u32) -> Self {
        let samples_per_pass = 4;
        Self {
            total_passes: (total_samples.div_ceil(samples_per_pass)),
            current_pass: 0,
            total_samples,
            samples_per_pass,
        }
    }

    pub fn update(&mut self, total_samples: u32) -> Self {
        self.total_samples = total_samples;
        self.total_passes = self.total_samples.div_ceil(self.samples_per_pass);
        *self
    }
}

pub struct GpuWrapper {
    buffer_wrapper: GpuBuffers,
    bind_group_wrapper: BindGroup,
    bind_group_layout_wrapper: BindGroupLayout,
    device: wgpu::Device,
    queue: wgpu::Queue,
    rc: RenderConfig,
    prh: ProgressiveRenderHelper,
    pipeline_wrapper: ComputePipeline,
    initialized: bool,
}

impl GpuWrapper {
    ///initializes shared Config, deligated to Sub modules
    pub fn new(rc: RenderConfig, path: &str) -> Result<Self> {
        let gpu = GpuDevice::new().unwrap();
        let initial_uniforms = match rc.uniforms {
            Change::Create(u) | Change::Update(u) => u,
            Change::Keep => Uniforms::default(),
            Change::Delete => panic!("Cannot create GpuWrapper with deleted uniforms"),
        };
        let prh = ProgressiveRenderHelper::new(initial_uniforms.total_samples);
        let buffers = GpuBuffers::new(&rc, &gpu.device, &prh);
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
            prh,
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
                    self.prh.update(uniforms.total_samples);
                    self.buffer_wrapper.grow_resolution(&self.device, new_size);
                }
            }
            if let Change::Create(spheres) = &new_rc.spheres {
                self.buffer_wrapper.init_spheres(&self.device, spheres);
            }
            if let Change::Create(vertices) = &new_rc.vertices {
                self.buffer_wrapper.init_vertices(&self.device, vertices);
            }
            if let Change::Create(uvs) = &new_rc.uvs {
                self.buffer_wrapper.init_uvs(&self.device, uvs);
            }
            if let Change::Create(triangles) = &new_rc.triangles {
                self.buffer_wrapper.init_triangles(&self.device, triangles);
            }
            if let Change::Create(meshes) = &new_rc.meshes {
                self.buffer_wrapper.init_meshes(&self.device, meshes);
            }
            if let Change::Create(lights) = &new_rc.lights {
                self.buffer_wrapper.init_lights(&self.device, lights);
            }
            if let Change::Create(textures) = &new_rc.textures {
                self.buffer_wrapper.init_textures(&self.device, textures);
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
                    self.prh.update(uniforms.total_samples);
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

            match &new_rc.uvs {
                Change::Keep => log::info!("Not updating UVs Buffer."),
                Change::Update(uvs) => {
                    self.buffer_wrapper.update_uvs(&self.device, uvs);
                }
                Change::Delete => {
                    self.buffer_wrapper.delete_uvs(&self.device);
                }
                Change::Create(_) => {
                    log::warn!("Create not allowed after initialization for uvs.");
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

            match &new_rc.meshes {
                Change::Keep => log::info!("Not updating Meshes Buffer."),
                Change::Update(meshes) => {
                    self.buffer_wrapper.update_meshes(&self.device, meshes);
                }
                Change::Delete => {
                    self.buffer_wrapper.delete_meshes(&self.device);
                }
                Change::Create(_) => {
                    log::warn!("Create not allowed after initialization for triangles.");
                }
            }
            match &new_rc.lights {
                Change::Keep => log::info!("Not updating Lights Buffer."),
                Change::Update(lights) => {
                    self.buffer_wrapper.update_lights(&self.device, lights);
                }
                Change::Delete => {
                    self.buffer_wrapper.delete_lights(&self.device);
                }
                Change::Create(_) => {
                    log::warn!("Create not allowed after initialization for lights.");
                }
            }

            match &new_rc.textures {
                Change::Keep => log::info!("Not updating Textures Buffer."),
                Change::Update(textures) => {
                    self.buffer_wrapper.update_textures(&self.device, textures);
                }
                Change::Delete => {
                    self.buffer_wrapper.delete_textures(&self.device);
                }
                Change::Create(_) => {
                    log::warn!("Create not allowed after initialization for textures.");
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

        for pass in 0..self.prh.total_passes {
            info!("Rendering pass {}/{}", pass + 1, self.prh.total_passes);
            self.prh.current_pass = pass;
            self.queue.write_buffer(
                &self.buffer_wrapper.progressive_render,
                0,
                bytemuck::cast_slice(&[self.prh]),
            );

            self.dispatch_compute_progressive(pass, self.prh.total_passes)?;
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

        match &self.rc.spheres {
            Change::Create(s) | Change::Update(s) => uniforms.spheres_count = s.len() as u32,
            Change::Delete => uniforms.spheres_count = 0,
            Change::Keep => {}
        };
        match &self.rc.triangles {
            Change::Create(t) | Change::Update(t) => uniforms.triangles_count = t.len() as u32 / 3,
            Change::Delete => uniforms.triangles_count = 0,
            Change::Keep => {}
        };

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

        self.queue.write_buffer(
            &self.buffer_wrapper.progressive_render,
            0,
            bytemuck::cast_slice(&[self.prh]),
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

        if let Change::Create(uvs) | Change::Update(uvs) = &self.rc.uvs {
            self.queue
                .write_buffer(&self.buffer_wrapper.uvs, 0, bytemuck::cast_slice(uvs));
        }

        if let Change::Create(triangles) | Change::Update(triangles) = &self.rc.triangles {
            self.queue.write_buffer(
                &self.buffer_wrapper.triangles,
                0,
                bytemuck::cast_slice(triangles),
            );
        }

        if let Change::Create(lights) | Change::Update(lights) = &self.rc.lights {
            self.queue
                .write_buffer(&self.buffer_wrapper.lights, 0, bytemuck::cast_slice(lights));
        }

        if let Change::Create(textures) | Change::Update(textures) = &self.rc.textures {
            let (tex_data, tex_info) = crate::buffers::GpuBuffers::process_textures(textures);
            self.queue.write_buffer(
                &self.buffer_wrapper.texture_data,
                0,
                bytemuck::cast_slice(&tex_data),
            );
            self.queue.write_buffer(
                &self.buffer_wrapper.texture_info,
                0,
                bytemuck::cast_slice(&tex_info),
            );
        }
    }
}
