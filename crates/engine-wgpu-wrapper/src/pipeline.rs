use std::fs;
use std::path::Path;

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
    /// It reads the shader source file from the path specified relative to the
    /// `engine-wgpu-wrapper` crate root (using `CARGO_MANIFEST_DIR`).
    ///
    /// # Arguments
    ///
    /// * `device` - The wgpu device.
    /// * `bind_group_layout` - The layout of the resources expected by the shader.
    /// * `path` - The relative path to the WGSL shader file (e.g., `"../engine-raytracer/src/shader.wgsl"`).
    ///
    /// # Panics
    ///
    /// Panics if the shader file cannot be read or if pipeline creation fails.
    pub fn new(
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
        path: &str,
    ) -> Self {
        // Pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[bind_group_layout],
            immediate_size: 0,
        });

        let path_new: &str = &(env!("CARGO_MANIFEST_DIR").to_owned() + "/../" + path);

        let shader_path = Path::new(path_new);

        let shader_source = fs::read_to_string(shader_path).unwrap();
        // .unwrap_or_else(|_| panic!("Failed to read shader file: {:?}", shader_path));

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&format!("{} shader", path)),
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
