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

const SAMPLES_PER_PASS: u32 = 1;

/// Helper struct for managing progressive rendering state.
///
/// It tracks the current pass, total passes, and sample counts to allow splitting
/// a heavy rendering task into smaller, manageable work units. This struct is passed
/// to the GPU as a uniform.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ProgressiveRenderHelper {
    /// Total number of passes required to complete the render.
    pub total_passes: u32,
    /// The current pass index (0-based).
    pub current_pass: u32,
    /// Total samples to be accumulated.
    pub total_samples: u32,
    /// Number of samples computed per pass.
    pub samples_per_pass: u32,
}

impl ProgressiveRenderHelper {
    /// Creates a new helper instance.
    ///
    /// # Arguments
    ///
    /// * `total_samples` - The total number of samples desired for the final image.
    pub fn new(total_samples: u32) -> Self {
        Self {
            total_passes: (total_samples.div_ceil(SAMPLES_PER_PASS)),
            current_pass: 0,
            total_samples,
            samples_per_pass: SAMPLES_PER_PASS,
        }
    }

    /// Updates the total sample count and recalculates the total passes.
    pub fn update(&mut self, total_samples: u32) -> Self {
        self.total_samples = total_samples;
        self.total_passes = self.total_samples.div_ceil(self.samples_per_pass);
        *self
    }
}

/// The main interface for WGPU-based rendering engines.
///
/// `GpuWrapper` orchestrates the interaction between the `RenderConfig` and the GPU.
/// It manages the lifecycle of GPU resources (buffers, bind groups), the compute pipeline,
/// and the execution of compute passes.
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
    /// Initializes the `GpuWrapper` and its sub-components.
    ///
    /// This delegates to `GpuDevice`, `GpuBuffers`, `BindGroupLayout`, and `ComputePipeline`
    /// to set up the full rendering context.
    ///
    /// # Arguments
    ///
    /// * `rc` - The initial render configuration. Must contain `Change::Create` for all required fields.
    /// * `path` - The path to the shader source file.
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

    /// Updates the GPU resources based on the new render configuration.
    ///
    /// This method handles:
    /// - Resizing buffers if the resolution changes.
    /// - Updating, deleting, or recreating buffers for scene objects (spheres, meshes, etc.).
    /// - Recreating the bind group if buffers are changed.
    ///
    /// It differentiates between the first initialization (where `Create` is expected)
    /// and subsequent updates (where `Update`, `Keep`, or `Delete` are used).
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
            if let Change::Create(uvs) = &new_rc.uvs {
                self.buffer_wrapper.init_uvs(&self.device, uvs);
            }
            if let Change::Create(meshes) = &new_rc.meshes {
                self.buffer_wrapper.init_meshes(&self.device, meshes);
            }
            if let Change::Create(lights) = &new_rc.lights {
                self.buffer_wrapper.init_lights(&self.device, lights);
            }
            if let Change::Create(bvh_nodes) = &new_rc.bvh_nodes {
                self.buffer_wrapper.init_bvh_nodes(&self.device, bvh_nodes);
            }
            if let Change::Create(bvh_indices) = &new_rc.bvh_indices {
                self.buffer_wrapper
                    .init_bvh_indices(&self.device, bvh_indices);
            }
            if let Change::Create(bvh_triangles) = &new_rc.bvh_triangles {
                self.buffer_wrapper
                    .init_bvh_triangles(&self.device, bvh_triangles);
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
            match &new_rc.bvh_nodes {
                Change::Keep => log::info!("Not updating BVH Nodes."),
                Change::Update(nodes) => {
                    self.buffer_wrapper.update_bvh_nodes(&self.device, nodes);
                }
                Change::Delete => {
                    self.buffer_wrapper.delete_bvh_nodes(&self.device);
                }
                Change::Create(nodes) => {
                    self.buffer_wrapper.update_bvh_nodes(&self.device, nodes);
                }
            }
            match &new_rc.bvh_indices {
                Change::Keep => log::info!("Not updating BVH Indices."),
                Change::Update(indices) => {
                    self.buffer_wrapper
                        .update_bvh_indices(&self.device, indices);
                }
                Change::Delete => {
                    self.buffer_wrapper.delete_bvh_indices(&self.device);
                }
                Change::Create(indices) => {
                    self.buffer_wrapper
                        .update_bvh_indices(&self.device, indices);
                }
            }
            match &new_rc.bvh_triangles {
                Change::Keep => log::info!("Not updating BVH Triangles."),
                Change::Update(tris) => {
                    self.buffer_wrapper.update_bvh_triangles(&self.device, tris);
                }
                Change::Delete => {
                    self.buffer_wrapper.delete_bvh_triangles(&self.device);
                }
                Change::Create(triangles) => {
                    self.buffer_wrapper
                        .update_bvh_triangles(&self.device, triangles);
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
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn buffer_wrapper(&self) -> &GpuBuffers {
        &self.buffer_wrapper
    }

    pub fn prh(&self) -> &ProgressiveRenderHelper {
        &self.prh
    }

    pub fn prh_mut(&mut self) -> &mut ProgressiveRenderHelper {
        &mut self.prh
    }

    /// Dispatches a single compute pass for progressive rendering.
    ///
    /// # Arguments
    ///
    /// * `pass_index` - The current pass number.
    /// * `total_passes` - The total number of passes.
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

    /// Dispatches all compute passes sequentially.
    ///
    /// This is a blocking operation that executes the full rendering process.
    /// It clears the accumulation buffer before starting.
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

    /// Reads the rendered pixels from the staging buffer.
    ///
    /// This method maps the staging buffer, reads the data, and returns it as a `Vec<u8>`.
    /// The returned data is in RGBA8 format (although the alpha channel is manually set to 255).
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
        let width = self.get_width() as usize;
        let height = self.get_height() as usize;
        let mut result = Vec::with_capacity(width * height * 4);

        for y in 0..height {
            for x in (0..width).rev() {
                let idx = (y * width + x) * 4;
                result.push(data_slice[idx]);
                result.push(data_slice[idx + 1]);
                result.push(data_slice[idx + 2]);
                result.push(255u8);
            }
        }

        drop(data_slice);
        self.buffer_wrapper.staging.unmap();
        Ok(result)
    }

    /// Updates the data in the GPU buffers with the values from the current `RenderConfig`.
    ///
    /// This method writes the CPU-side data to the corresponding GPU buffers.
    /// It should be called before dispatching compute shaders if the scene has changed.
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

        match &self.rc.bvh_nodes {
            Change::Create(n) | Change::Update(n) => {
                uniforms.bvh_node_count = n.len() as u32;
            }
            Change::Delete => uniforms.bvh_node_count = 0,
            Change::Keep => {}
        }

        match &self.rc.bvh_triangles {
            Change::Create(t) | Change::Update(t) => {
                uniforms.bvh_triangle_count = t.len() as u32;
            }
            Change::Delete => uniforms.bvh_triangle_count = 0,
            Change::Keep => {}
        }

        log::info!(
            "Writing uniforms to GPU: camera_pos={:?}, camera_dir={:?}, pane_distance={}, pane_width={}, size={}x{}, spheres={}, triangles={}",
            uniforms.camera.pos,
            uniforms.camera.dir,
            uniforms.camera.pane_distance,
            uniforms.camera.pane_width,
            uniforms.width,
            uniforms.height,
            uniforms.spheres_count,
            uniforms.bvh_triangle_count
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

        if let Change::Create(uvs) | Change::Update(uvs) = &self.rc.uvs {
            self.queue
                .write_buffer(&self.buffer_wrapper.uvs, 0, bytemuck::cast_slice(uvs));
        }

        if let Change::Create(lights) | Change::Update(lights) = &self.rc.lights {
            self.queue
                .write_buffer(&self.buffer_wrapper.lights, 0, bytemuck::cast_slice(lights));
        }

        if let Change::Create(nodes) | Change::Update(nodes) = &self.rc.bvh_nodes {
            self.queue.write_buffer(
                &self.buffer_wrapper.bvh_nodes,
                0,
                bytemuck::cast_slice(nodes),
            );
        }

        if let Change::Create(indices) | Change::Update(indices) = &self.rc.bvh_indices {
            self.queue.write_buffer(
                &self.buffer_wrapper.bvh_indices,
                0,
                bytemuck::cast_slice(indices),
            );
        }

        if let Change::Create(tris) | Change::Update(tris) = &self.rc.bvh_triangles {
            self.queue.write_buffer(
                &self.buffer_wrapper.bvh_triangles,
                0,
                bytemuck::cast_slice(tris),
            );
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
