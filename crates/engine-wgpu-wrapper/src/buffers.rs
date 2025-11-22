use engine_config::{RenderConfig, Uniforms};
use wgpu::util::DeviceExt;
use wgpu::{Buffer, Device};

pub struct GpuBuffers {
    pub spheres: Buffer,
    pub uniforms: Buffer,
    pub output: Buffer,
    pub staging: Buffer,
    pub verticies: Buffer,
    pub triangles: Buffer,
}

impl GpuBuffers {
    pub fn new(rc: &RenderConfig, device: &Device) -> Self {
        let size = (rc.uniforms.width * rc.uniforms.height * 4) as u64;

        Self {
            spheres: Self::create_storage_buffer(device, "Spheres Buffer", &rc.spheres),
            uniforms: Self::create_uniform_buffer(device, "Uniforms Buffer", &rc.uniforms),
            output: Self::create_output_buffer(device, size),
            staging: Self::create_staging_buffer(device, size),
            verticies: Self::create_storage_buffer(device, "Verticies Buffer", &rc.verticies),
            triangles: Self::create_storage_buffer(device, "Triangles Buffer", &rc.triangles),
        }
    }

    pub fn grow_resolution(&mut self, device: &Device, size: u64) {
        self.output = Self::create_output_buffer(device, size);
        self.staging = Self::create_staging_buffer(device, size);
    }

    pub fn grow_spheres(&mut self, device: &Device, rc: &RenderConfig) {
        self.spheres = Self::create_storage_buffer(device, "Spheres Buffer", &rc.spheres);
    }

    pub fn grow_verticies(&mut self, device: &Device, rc: &RenderConfig) {
        self.verticies = Self::create_storage_buffer(device, "Verticies Buffer", &rc.verticies);
    }

    pub fn grow_triangles(&mut self, device: &Device, rc: &RenderConfig) {
        self.triangles = Self::create_storage_buffer(device, "Triangles Buffer", &rc.triangles);
    }

    fn create_uniform_buffer(device: &Device, label: &str, data: &Uniforms) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents: bytemuck::bytes_of(data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }

    fn create_storage_buffer<T: bytemuck::Pod>(device: &Device, label: &str, data: &[T]) -> Buffer {
        if data.is_empty() {
            let size = std::mem::size_of::<T>() as u64;
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: if size == 0 { 4 } else { size }, // Handle ZSTs, though Pod shouldn't be ZSTs.
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        } else {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents: bytemuck::cast_slice(data),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            })
        }
    }

    fn create_output_buffer(device: &Device, size: u64) -> Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output Buffer"),
            size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        })
    }

    fn create_staging_buffer(device: &Device, size: u64) -> Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }
}
