use engine_config::{RenderConfig, Uniforms, Sphere, PointLight};
use engine_config::render_config::Change;
use wgpu::util::DeviceExt;
use wgpu::{Buffer, Device};
use crate::ProgressiveRenderHelper;

pub struct GpuBuffers {
    pub spheres: Buffer,
    pub uniforms: Buffer,
    pub output: Buffer,
    pub staging: Buffer,
    pub vertices: Buffer,
    pub triangles: Buffer,
    pub accumulation: Buffer,
    pub progressive_render: Buffer,
    pub lights: Buffer,
}

impl GpuBuffers {
    pub fn new(rc: &RenderConfig, device: &Device, prh: &ProgressiveRenderHelper) -> Self {
        let uniforms = match &rc.uniforms {
            Change::Create(u) => u,
            _ => panic!("Uniforms must be Create during initialization"),
        };
        let spheres = match &rc.spheres {
            Change::Create(s) => s.as_slice(),
            _ => panic!("Spheres must be Create during initialization"),
        };
        let vertices = match &rc.vertices {
            Change::Create(v) => v.as_slice(),
            _ => panic!("Vertices must be Create during initialization"),
        };
        let triangles = match &rc.triangles {
            Change::Create(t) => t.as_slice(),
            _ => panic!("Triangles must be Create during initialization"),
        };
        let lights = match &rc.lights {
            Change::Create(l) => l.as_slice(),
            _ => panic!("Lights must be Create during initialization"),
        };

        let size = (uniforms.width * uniforms.height * 4) as u64;

        // Add this when creating buffers
        let accumulation_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Accumulation Buffer"),
            size: (uniforms.width * uniforms.height * 16) as u64, // vec4<f32> = 16 bytes
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            spheres: Self::create_storage_buffer(device, "Spheres Buffer", spheres),
            uniforms: Self::create_uniform_buffer(device, "Uniforms Buffer", uniforms),
            output: Self::create_output_buffer(device, size),
            staging: Self::create_staging_buffer(device, size),
            vertices: Self::create_storage_buffer(device, "Vertices Buffer", vertices),
            triangles: Self::create_storage_buffer(device, "Triangles Buffer", triangles),
            accumulation: accumulation_buffer,
            progressive_render: Self::create_uniform_buffer(
                device,
                "Progressive Render Buffer",
                &[*prh],
            ),
            lights: Self::create_storage_buffer(device, "Light Buffer", lights),
        }
    }

    pub fn grow_resolution(&mut self, device: &Device, size: u64) {
        self.output = Self::create_output_buffer(device, size);
        self.staging = Self::create_staging_buffer(device, size);
        self.accumulation = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Accumulation Buffer"),
            size: (size * 4),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
    }

    pub fn grow_spheres(&mut self, device: &Device, spheres: &[Sphere]) {
        self.spheres = Self::create_storage_buffer(device, "Spheres Buffer", spheres);
    }

    pub fn grow_vertices(&mut self, device: &Device, vertices: &[f32]) {
        self.vertices = Self::create_storage_buffer(device, "Vertices Buffer", vertices);
    }

    pub fn grow_triangles(&mut self, device: &Device, triangles: &[u32]) {
        self.triangles = Self::create_storage_buffer(device, "Triangles Buffer", triangles);
    }

    pub fn grow_lights(&mut self, device: &Device, lights: &[PointLight]) {
        self.lights = Self::create_storage_buffer(device, "Lights Buffer", lights);
    }

    fn create_uniform_buffer<T: bytemuck::Pod>(device: &Device, label: &str, data: &T) -> Buffer {
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

    // Init methods for first-time creation
    pub fn init_uniforms(&mut self, device: &Device, uniforms: &Uniforms) {
        self.uniforms = Self::create_uniform_buffer(device, "Uniforms Buffer", uniforms);
    }

    pub fn init_spheres(&mut self, device: &Device, spheres: &[Sphere]) {
        self.spheres = Self::create_storage_buffer(device, "Spheres Buffer", spheres);
    }

    pub fn init_vertices(&mut self, device: &Device, vertices: &[f32]) {
        self.vertices = Self::create_storage_buffer(device, "Vertices Buffer", vertices);
    }

    pub fn init_triangles(&mut self, device: &Device, triangles: &[u32]) {
        self.triangles = Self::create_storage_buffer(device, "Triangles Buffer", triangles);
    }

    pub fn init_lights(&mut self, device: &Device, lights: &[PointLight]) {
        self.lights = Self::create_storage_buffer(device, "Lights Buffer", lights);
    }

    // Update methods for existing buffers
    pub fn update_uniforms(&mut self, device: &Device, uniforms: &Uniforms) {
        self.uniforms = Self::create_uniform_buffer(device, "Uniforms Buffer", uniforms);
    }

    pub fn update_spheres(&mut self, device: &Device, spheres: &[Sphere]) {
        self.spheres = Self::create_storage_buffer(device, "Spheres Buffer", spheres);
    }

    pub fn update_vertices(&mut self, device: &Device, vertices: &[f32]) {
        self.vertices = Self::create_storage_buffer(device, "Vertices Buffer", vertices);
    }

    pub fn update_triangles(&mut self, device: &Device, triangles: &[u32]) {
        self.triangles = Self::create_storage_buffer(device, "Triangles Buffer", triangles);
    }

    pub fn update_lights(&mut self, device: &Device, lights: &[PointLight]) {
        self.lights = Self::create_storage_buffer(device, "Lights Buffer", lights);
    }

    // Delete methods (create minimal empty buffers)
    pub fn delete_uniforms(&mut self, device: &Device) {
        let dummy_uniforms = Uniforms::default();
        self.uniforms =
            Self::create_uniform_buffer(device, "Uniforms Buffer (deleted)", &dummy_uniforms);
    }

    pub fn delete_spheres(&mut self, device: &Device) {
        self.spheres =
            Self::create_storage_buffer(device, "Spheres Buffer (deleted)", &[] as &[Sphere]);
    }

    pub fn delete_vertices(&mut self, device: &Device) {
        self.vertices =
            Self::create_storage_buffer(device, "Vertices Buffer (deleted)", &[] as &[f32]);
    }

    pub fn delete_triangles(&mut self, device: &Device) {
        self.triangles =
            Self::create_storage_buffer(device, "Triangles Buffer (deleted)", &[] as &[u32]);
    }

    pub fn delete_lights(&mut self, device: &Device) {
        self.lights = Self::create_storage_buffer(device, "Lights Buffer (deleted)", &[] as &[u32]);
    }
}
