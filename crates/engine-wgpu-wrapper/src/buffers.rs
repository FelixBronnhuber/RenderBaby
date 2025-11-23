use engine_config::RenderConfig;
use wgpu::util::DeviceExt;
use wgpu::{Buffer, Device};

pub struct GpuBuffers {
    pub spheres: Buffer,
    pub camera: Buffer,
    pub output: Buffer,
    pub staging: Buffer,
    pub verticies: Buffer,
    pub triangles: Buffer,
}

impl GpuBuffers {
    pub fn new(rc: &RenderConfig, device: &Device) -> Self {
        let size = (rc.camera.width * rc.camera.height * 4) as u64;

        Self {
            spheres: Self::create_storage_buffer(device, "Spheres Buffer", &rc.spheres),
            camera: Self::create_uniform_buffer(device, &rc.camera),
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

    fn create_uniform_buffer<T: bytemuck::Pod>(device: &Device, data: &T) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::bytes_of(data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }

    fn create_storage_buffer<T: bytemuck::Pod>(device: &Device, label: &str, data: &[T]) -> Buffer {
        if data.is_empty() {
            Self::create_placeholder_buffer(device, label)
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

    fn create_placeholder_buffer(device: &Device, label: &str) -> Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: 4,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }
}
