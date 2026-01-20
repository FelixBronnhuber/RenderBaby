/// Wrapper for the `wgpu::ComputePipeline`.
///
/// This struct handles the loading of the WGSL shader source code and the creation
/// of the compute pipeline using the provided bind group layout.
pub struct ComputePipeline {
    pub pipeline: wgpu::ComputePipeline,
}

impl ComputePipeline {
    /// Creates a new compute pipeline.
    ///
    /// It uses the provided WGSL shader source code.
    ///
    /// # Arguments
    ///
    /// * `device` - The wgpu device.
    /// * `bind_group_layout` - The layout of the resources expected by the shader.
    /// * `shader_source` - The WGSL shader source code.
    ///
    /// # Panics
    ///
    /// Panics if pipeline creation fails.
    pub fn new(
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
        shader_source: &str,
    ) -> Self {
        // Pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[bind_group_layout],
            immediate_size: 0,
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

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
