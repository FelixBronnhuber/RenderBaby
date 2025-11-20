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
        let cam = rc.camera;
        let size = (rc.camera.width * rc.camera.height * 4) as u64;

        let dimensions_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Dimensions Buffer"),
            contents: bytemuck::bytes_of(&cam),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let mut spheres_buffer = Self::create_placeholder_buffer(device);

        if !rc.spheres.is_empty() {
            spheres_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Spheres Buffer"),
                contents: bytemuck::cast_slice(&rc.spheres),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            });
        }

        let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output Buffer"),
            size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut verticies_buffer = Self::create_placeholder_buffer(device);

        if !rc.verticies.is_empty() {
            verticies_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Verticies Buffer"),
                contents: bytemuck::cast_slice(&rc.verticies),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            });
        }

        let mut triangles_buffer = Self::create_placeholder_buffer(device);

        if !rc.triangles.is_empty() {
            triangles_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Triangles Buffer"),
                contents: bytemuck::cast_slice(&rc.triangles),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            });
        }

        Self {
            spheres: spheres_buffer,
            camera: dimensions_buffer,
            output: output_buffer,
            staging: staging_buffer,
            verticies: verticies_buffer,
            triangles: triangles_buffer,
        }
    }

    pub fn grow_resolution(&mut self, device: &Device, size: u64) {
        self.output = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output Buffer"),
            size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        self.staging = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
    }

    pub fn grow_spheres(&mut self, device: &Device, rc: &RenderConfig) {
        self.spheres = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Spheres Buffer"),
            contents: bytemuck::cast_slice(&rc.spheres),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });
    }

    pub fn grow_verticies(&mut self, device: &Device, rc: &RenderConfig) {
        self.verticies = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Verticies Buffer"),
            contents: bytemuck::cast_slice(&rc.verticies),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });
    }

    pub fn grow_triangles(&mut self, device: &Device, rc: &RenderConfig) {
        self.triangles = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Triangles Buffer"),
            contents: bytemuck::cast_slice(&rc.triangles),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });
    }

    fn create_placeholder_buffer(device: &Device) -> Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Spheres Buffer"),
            size: 4,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }
}
