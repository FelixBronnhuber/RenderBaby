use anyhow::{Result, anyhow};
use engine_config::{Camera, RenderCommand, Sphere};
pub use engine_raytracer::{RenderOutput, RenderState};
use std::sync::Arc;
use wgpu::util::DeviceExt;

pub const SPHERES: [Sphere; 5] = [
    Sphere::new([0.0, 0.6, 1.0], 0.5, [1.0, 0.0, 1.0]), // Top, magenta
    Sphere::new([-0.6, 0.0, 1.0], 0.5, [0.0, 1.0, 0.0]), // Left, green
    Sphere::new([0.0, 0.0, 1.0], 0.5, [1.0, 0.0, 0.0]), // Centered, red
    Sphere::new([0.6, 0.0, 1.0], 0.5, [0.0, 0.0, 1.0]), // Right, blue
    Sphere::new([0.0, -0.6, 1.0], 0.5, [0.0, 1.0, 1.0]), // Bottom, cyan
];

pub struct GpuDevice {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
}

impl GpuDevice {
    pub fn new() -> Result<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter_result =
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            }));

        let adapter = match adapter_result {
            Ok(adapter) => adapter,
            Err(_) => return Err(anyhow!("WGPU: no suitable GPU adapter found")),
        };

        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("Render device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            ..Default::default()
        }))
        .map_err(|e| anyhow!("WGPU: failed to create device/queue: {}", e))?;

        Ok(Self {
            device: Arc::new(device),
            queue: Arc::new(queue),
        })
    }
}

pub struct BufferBundle {
    pub dimensions: wgpu::Buffer,
    pub spheres: wgpu::Buffer,
    pub output: wgpu::Buffer,
    pub staging: wgpu::Buffer,
}

impl BufferBundle {
    pub fn new(device: &wgpu::Device, width: u32, height: u32, fov: f32) -> Self {
        let dimensions = Camera { width, height, fov };

        let dimensions_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Dimensions Buffer"),
            contents: bytemuck::bytes_of(&dimensions),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let spheres_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Spheres Buffer"),
            contents: bytemuck::cast_slice(SPHERES.as_ref()),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output Buffer"),
            size: (width * height * 4) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: (width * height * 4) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            dimensions: dimensions_buffer,
            spheres: spheres_buffer,
            output: output_buffer,
            staging: staging_buffer,
        }
    }
}

pub struct BindGroupBundle {
    pub layout: wgpu::BindGroupLayout,
    pub group: wgpu::BindGroup,
}

impl BindGroupBundle {
    pub fn new(device: &wgpu::Device, buffers: &BufferBundle) -> Self {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Main Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Main Bind Group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffers.dimensions.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: buffers.output.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: buffers.spheres.as_entire_binding(),
                },
            ],
        });

        Self { layout, group }
    }
}

pub struct WgpuContext {
    pub gpu: GpuDevice,
    pub buffers: BufferBundle,
    pub bind_groups: BindGroupBundle,
    pub width: u32,
    pub height: u32,
}

impl WgpuContext {
    pub fn new(width: u32, height: u32, fov: f32) -> Result<Self> {
        let gpu = GpuDevice::new()?;
        let buffers = BufferBundle::new(&gpu.device, width, height, fov);
        let bind_groups = BindGroupBundle::new(&gpu.device, &buffers);

        Ok(Self {
            gpu,
            buffers,
            bind_groups,
            width,
            height,
        })
    }
}

pub trait Renderer {
    fn render(&mut self, rc: RenderCommand) -> Result<RenderOutput>;
    fn update(&mut self, rc: RenderCommand) -> Result<RenderOutput>;
}

pub struct WgpuWrapper {
    renderer: RenderState,
}

pub enum EngineType {
    Raytracer,
    Pathtracer,
}

impl WgpuWrapper {
    pub fn new(engine_type: EngineType, width: usize, height: usize, fov: f32) -> Result<Self> {
        let ctx = WgpuContext::new(width as u32, height as u32, fov)?;

        let renderer = match engine_type {
            EngineType::Raytracer => RenderState::new(
                Arc::clone(&ctx.gpu.device),
                Arc::clone(&ctx.gpu.queue),
                ctx.buffers.output,
                ctx.buffers.staging,
                ctx.bind_groups.layout,
                ctx.bind_groups.group,
                ctx.buffers.dimensions,
                ctx.buffers.spheres,
                (width, height),
            ),
            EngineType::Pathtracer => todo!("Implement PathTracer::new(...)"),
        };

        Ok(Self { renderer })
    }

    pub fn render(&mut self, rc: RenderCommand) -> Result<RenderOutput> {
        self.renderer.render(rc)
    }
}
