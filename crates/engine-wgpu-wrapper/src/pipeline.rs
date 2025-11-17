use std::fs;
use std::path::Path;

pub struct ComputePipeline {
    pub pipeline: wgpu::ComputePipeline,
}

impl ComputePipeline {
    pub fn new(device: &wgpu::Device, bind_group_layout: &wgpu::BindGroupLayout, path: &str) -> Self {
        // Pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[bind_group_layout],
            push_constant_ranges: &[],
        });

        //let shader = device.create_shader_module(wgpu::include_wgsl!("../../engine-raytracer/shader.wgsl"));

        let path_new:&str = &(env!("CARGO_MANIFEST_DIR").to_owned() + "/../" + path);

        let shader_path = Path::new(path_new);

        let shader_source = fs::read_to_string(&shader_path)
            .expect(&format!("Failed to read shader file: {:?}", shader_path));

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
