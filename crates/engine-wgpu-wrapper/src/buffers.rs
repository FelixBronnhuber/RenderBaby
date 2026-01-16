use engine_config::{Mesh, RenderConfig, Sphere, Uniforms, PointLight, TextureData};
use engine_config::render_config::Change;
use wgpu::util::DeviceExt;
use wgpu::{Buffer, Device};
use crate::ProgressiveRenderHelper;
use engine_bvh::bvh::BVHNode;
use engine_bvh::triangle::GPUTriangle;
use bytemuck::{Pod, Zeroable};

/// Metadata for a texture stored in the global texture data buffer.
///
/// Since all textures are flattened into a single byte array, this struct
/// provides the necessary information to index into that array.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct TextureInfo {
    /// Offset in bytes into the `texture_data` buffer where this texture starts.
    pub offset: u32,
    /// Width of the texture in pixels.
    pub width: u32,
    /// Height of the texture in pixels.
    pub height: u32,
    /// Padding to ensure 16-byte alignment (required for some GPU structures).
    pub _pad: u32,
}

/// Manages all GPU buffers used by the rendering engine.
///
/// This includes storage buffers for scene geometry (spheres, meshes), BVH structures,
/// lights, and textures, as well as uniform buffers for camera and render settings.
/// It also handles the output, staging, and accumulation buffers for the rendering process.
pub struct GpuBuffers {
    /// Storage buffer containing sphere definitions.
    pub spheres: Buffer,
    /// Uniform buffer containing global render settings (camera, resolution, etc.).
    pub uniforms: Buffer,
    /// Storage buffer for the compute shader output (write-only for shader).
    pub output: Buffer,
    /// Staging buffer for reading back the output to the CPU (read-only for CPU).
    pub staging: Buffer,
    /// Storage buffer containing UV coordinates.
    pub uvs: Buffer,
    /// Storage buffer containing mesh definitions.
    pub meshes: Buffer,
    /// Storage buffer for accumulating samples over multiple frames (progressive rendering).
    pub accumulation: Buffer,
    /// Uniform buffer for progressive rendering state (pass count, etc.).
    pub progressive_render: Buffer,
    /// Storage buffer containing point light definitions.
    pub lights: Buffer,
    /// Storage buffer for BVH nodes.
    pub bvh_nodes: Buffer,
    /// Storage buffer for BVH indices (references to triangles).
    pub bvh_indices: Buffer,
    /// Storage buffer for BVH triangles (geometry data).
    pub bvh_triangles: Buffer,
    /// Storage buffer containing raw texture pixel data (flattened).
    pub texture_data: Buffer,
    /// Storage buffer containing metadata for accessing textures in `texture_data`.
    pub texture_info: Buffer,
}

impl GpuBuffers {
    /// Creates a new set of GPU buffers based on the provided `RenderConfig`.
    ///
    /// This method initializes all buffers with the data from the configuration.
    /// It expects the configuration fields to be in the `Change::Create` state.
    ///
    /// # Arguments
    ///
    /// * `rc` - The initial render configuration.
    /// * `device` - The WGPU device used to create the buffers.
    /// * `prh` - Helper for progressive rendering state.
    pub fn new(rc: &RenderConfig, device: &Device, prh: &ProgressiveRenderHelper) -> Self {
        let uniforms = match &rc.uniforms {
            Change::Create(u) => u,
            _ => panic!("Uniforms must be Create during initialization"),
        };
        let spheres = match &rc.spheres {
            Change::Create(s) => s.as_slice(),
            _ => panic!("Spheres must be Create during initialization"),
        };
        let uvs = match &rc.uvs {
            Change::Create(v) => v.as_slice(),
            _ => panic!("UVs must be Create during initialization"),
        };
        let meshes = match &rc.meshes {
            Change::Create(t) => t.as_slice(),
            _ => panic!("Meshes must be Create during initialization"),
        };
        let lights = match &rc.lights {
            Change::Create(l) => l.as_slice(),
            _ => panic!("Lights must be Create during initialization"),
        };
        let textures = match &rc.textures {
            Change::Create(t) => t,
            _ => panic!("Textures must be Create during initialization"),
        };

        let (tex_data, tex_info) = Self::process_textures(textures);
        let bvh_nodes = match &rc.bvh_nodes {
            Change::Create(n) => n.as_slice(),
            _ => &[],
        };
        let bvh_indices = match &rc.bvh_indices {
            Change::Create(i) => i.as_slice(),
            _ => &[],
        };
        let bvh_triangles = match &rc.bvh_triangles {
            Change::Create(t) => t.as_slice(),
            _ => &[],
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
            uvs: Self::create_storage_buffer(device, "UVs Buffer", uvs),
            meshes: Self::create_storage_buffer(device, "Meshes Buffer", meshes),
            accumulation: accumulation_buffer,
            progressive_render: Self::create_uniform_buffer(
                device,
                "Progressive Render Buffer",
                &[*prh],
            ),
            lights: Self::create_storage_buffer(device, "Light Buffer", lights),
            bvh_nodes: Self::create_storage_buffer(device, "BVH Nodes Buffer", bvh_nodes),
            bvh_indices: Self::create_storage_buffer(device, "BVH Indices Buffer", bvh_indices),
            bvh_triangles: Self::create_storage_buffer(
                device,
                "BVH Triangles Buffer",
                bvh_triangles,
            ),
            texture_data: Self::create_storage_buffer(device, "Texture Data Buffer", &tex_data),
            texture_info: Self::create_storage_buffer(device, "Texture Info Buffer", &tex_info),
        }
    }

    /// Flattens a list of textures into a single data vector and a corresponding info vector.
    pub fn process_textures(textures: &[TextureData]) -> (Vec<u32>, Vec<TextureInfo>) {
        let mut data = Vec::new();
        let mut info = Vec::new();
        let mut offset = 0;

        for tex in textures {
            info.push(TextureInfo {
                offset,
                width: tex.width,
                height: tex.height,
                _pad: 0,
            });
            data.extend_from_slice(&tex.rgba_data);
            offset += tex.width * tex.height;
        }

        (data, info)
    }

    /// Recreates the output, staging, and accumulation buffers to match a new resolution.
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

    /// Recreates the spheres buffer with new data.
    pub fn grow_spheres(&mut self, device: &Device, spheres: &[Sphere]) {
        self.spheres = Self::create_storage_buffer(device, "Spheres Buffer", spheres);
    }

    /// Recreates the UVs buffer with new data.
    pub fn grow_uvs(&mut self, device: &Device, uvs: &[f32]) {
        self.uvs = Self::create_storage_buffer(device, "UVs Buffer", uvs);
    }

    /// Recreates the meshes buffer with new data.
    pub fn grow_meshes(&mut self, device: &Device, meshes: &[Mesh]) {
        self.meshes = Self::create_storage_buffer(device, "Meshes Buffer", meshes);
    }

    /// Recreates the lights buffer with new data.
    pub fn grow_lights(&mut self, device: &Device, lights: &[PointLight]) {
        self.lights = Self::create_storage_buffer(device, "Lights Buffer", lights);
    }

    /// Recreates the BVH nodes buffer with new data.
    pub fn grow_bvh_nodes(&mut self, device: &Device, nodes: &[BVHNode]) {
        self.bvh_nodes = Self::create_storage_buffer(device, "BVH Nodes Buffer", nodes);
    }

    /// Recreates the BVH indices buffer with new data.
    pub fn grow_bvh_indices(&mut self, device: &Device, indices: &[u32]) {
        self.bvh_indices = Self::create_storage_buffer(device, "BVH Indices Buffer", indices);
    }

    /// Recreates the BVH triangles buffer with new data.
    pub fn grow_bvh_triangles(&mut self, device: &Device, triangles: &[GPUTriangle]) {
        self.bvh_triangles = Self::create_storage_buffer(device, "BVH Triangles Buffer", triangles);
    }

    /// Recreates the texture data and info buffers with new texture data.
    pub fn grow_textures(&mut self, device: &Device, textures: &[TextureData]) {
        let (tex_data, tex_info) = Self::process_textures(textures);
        self.texture_data = Self::create_storage_buffer(device, "Texture Data Buffer", &tex_data);
        self.texture_info = Self::create_storage_buffer(device, "Texture Info Buffer", &tex_info);
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
    /// Initializes the uniforms buffer.
    pub fn init_uniforms(&mut self, device: &Device, uniforms: &Uniforms) {
        self.uniforms = Self::create_uniform_buffer(device, "Uniforms Buffer", uniforms);
    }

    /// Initializes the spheres buffer.
    pub fn init_spheres(&mut self, device: &Device, spheres: &[Sphere]) {
        self.spheres = Self::create_storage_buffer(device, "Spheres Buffer", spheres);
    }

    /// Initializes the UVs buffer.
    pub fn init_uvs(&mut self, device: &Device, uvs: &[f32]) {
        self.uvs = Self::create_storage_buffer(device, "UVs Buffer", uvs);
    }

    /// Initializes the meshes buffer.
    pub fn init_meshes(&mut self, device: &Device, meshes: &[Mesh]) {
        self.meshes = Self::create_storage_buffer(device, "Meshes Buffer", meshes);
    }

    /// Initializes the lights buffer.
    pub fn init_lights(&mut self, device: &Device, lights: &[PointLight]) {
        self.lights = Self::create_storage_buffer(device, "Lights Buffer", lights);
    }

    /// Initializes the BVH nodes buffer.
    pub fn init_bvh_nodes(&mut self, device: &Device, nodes: &[BVHNode]) {
        self.bvh_nodes = Self::create_storage_buffer(device, "BVH Nodes Buffer", nodes);
    }

    /// Initializes the BVH indices buffer.
    pub fn init_bvh_indices(&mut self, device: &Device, indices: &[u32]) {
        self.bvh_indices = Self::create_storage_buffer(device, "BVH Indices Buffer", indices);
    }

    /// Initializes the BVH triangles buffer.
    pub fn init_bvh_triangles(&mut self, device: &Device, triangles: &[GPUTriangle]) {
        self.bvh_triangles = Self::create_storage_buffer(device, "BVH Triangles Buffer", triangles);
    }

    /// Initializes the texture buffers.
    pub fn init_textures(&mut self, device: &Device, textures: &[TextureData]) {
        let (tex_data, tex_info) = Self::process_textures(textures);
        self.texture_data = Self::create_storage_buffer(device, "Texture Data Buffer", &tex_data);
        self.texture_info = Self::create_storage_buffer(device, "Texture Info Buffer", &tex_info);
    }

    // Update methods for existing buffers
    /// Updates the uniforms buffer by recreating it.
    pub fn update_uniforms(&mut self, device: &Device, uniforms: &Uniforms) {
        self.uniforms = Self::create_uniform_buffer(device, "Uniforms Buffer", uniforms);
    }

    /// Updates the spheres buffer by recreating it.
    pub fn update_spheres(&mut self, device: &Device, spheres: &[Sphere]) {
        self.spheres = Self::create_storage_buffer(device, "Spheres Buffer", spheres);
    }

    /// Updates the UVs buffer by recreating it.
    pub fn update_uvs(&mut self, device: &Device, uvs: &[f32]) {
        self.uvs = Self::create_storage_buffer(device, "UVs Buffer", uvs);
    }

    /// Updates the meshes buffer by recreating it.
    pub fn update_meshes(&mut self, device: &Device, meshes: &[Mesh]) {
        self.meshes = Self::create_storage_buffer(device, "Meshes Buffer", meshes);
    }

    /// Updates the lights buffer by recreating it.
    pub fn update_lights(&mut self, device: &Device, lights: &[PointLight]) {
        self.lights = Self::create_storage_buffer(device, "Lights Buffer", lights);
    }

    /// Updates the BVH nodes buffer by recreating it.
    pub fn update_bvh_nodes(&mut self, device: &Device, nodes: &[BVHNode]) {
        self.bvh_nodes = Self::create_storage_buffer(device, "BVH Nodes Buffer", nodes);
    }

    /// Updates the BVH indices buffer by recreating it.
    pub fn update_bvh_indices(&mut self, device: &Device, indices: &[u32]) {
        self.bvh_indices = Self::create_storage_buffer(device, "BVH Indices Buffer", indices);
    }

    /// Updates the BVH triangles buffer by recreating it.
    pub fn update_bvh_triangles(&mut self, device: &Device, triangles: &[GPUTriangle]) {
        self.bvh_triangles = Self::create_storage_buffer(device, "BVH Triangles Buffer", triangles);
    }

    /// Updates the texture buffers by recreating them.
    pub fn update_textures(&mut self, device: &Device, textures: &[TextureData]) {
        let (tex_data, tex_info) = Self::process_textures(textures);
        self.texture_data = Self::create_storage_buffer(device, "Texture Data Buffer", &tex_data);
        self.texture_info = Self::create_storage_buffer(device, "Texture Info Buffer", &tex_info);
    }

    // Delete methods (create minimal empty buffers)
    /// Replaces the uniforms buffer with a default/dummy one.
    pub fn delete_uniforms(&mut self, device: &Device) {
        let dummy_uniforms = Uniforms::default();
        self.uniforms =
            Self::create_uniform_buffer(device, "Uniforms Buffer (deleted)", &dummy_uniforms);
    }

    /// Replaces the spheres buffer with an empty one.
    pub fn delete_spheres(&mut self, device: &Device) {
        self.spheres =
            Self::create_storage_buffer(device, "Spheres Buffer (deleted)", &[] as &[Sphere]);
    }

    /// Replaces the UVs buffer with an empty one.
    pub fn delete_uvs(&mut self, device: &Device) {
        self.uvs = Self::create_storage_buffer(device, "UVs Buffer (deleted)", &[] as &[f32]);
    }

    /// Replaces the meshes buffer with an empty one.
    pub fn delete_meshes(&mut self, device: &Device) {
        self.meshes =
            Self::create_storage_buffer(device, "Meshes Buffer (deleted)", &[] as &[Mesh]);
    }

    /// Replaces the lights buffer with an empty one.
    pub fn delete_lights(&mut self, device: &Device) {
        self.lights = Self::create_storage_buffer(device, "Lights Buffer (deleted)", &[] as &[u32]);
    }

    /// Replaces the BVH nodes buffer with an empty one.
    pub fn delete_bvh_nodes(&mut self, device: &Device) {
        self.bvh_nodes =
            Self::create_storage_buffer(device, "BVH Nodes Buffer (deleted)", &[] as &[BVHNode]);
    }

    /// Replaces the BVH indices buffer with an empty one.
    pub fn delete_bvh_indices(&mut self, device: &Device) {
        self.bvh_indices =
            Self::create_storage_buffer(device, "BVH Indices Buffer (deleted)", &[] as &[u32]);
    }

    /// Replaces the BVH triangles buffer with an empty one.
    pub fn delete_bvh_triangles(&mut self, device: &Device) {
        self.bvh_triangles = Self::create_storage_buffer(
            device,
            "BVH Triangles Buffer (deleted)",
            &[] as &[GPUTriangle],
        );
    }

    /// Replaces the texture buffers with empty ones.
    pub fn delete_textures(&mut self, device: &Device) {
        self.texture_data =
            Self::create_storage_buffer(device, "Texture Data Buffer (deleted)", &[] as &[u32]);
        self.texture_info = Self::create_storage_buffer(
            device,
            "Texture Info Buffer (deleted)",
            &[] as &[TextureInfo],
        );
    }
}
