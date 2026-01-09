/// Serves as an adpter between the scene plane and the render engine.
use std::collections::HashMap;
use anyhow::{Error, Result};
use engine_config::{RenderConfigBuilder, RenderOutput};
use glam::Vec3;
use log::{debug, error, info};
use scene_objects::{
    camera::{Camera, Resolution},
    light_source::LightSource,
    mesh::Mesh,
    sphere::Sphere,
};
use crate::data_plane::scene::{render_scene::Scene};
use engine_bvh::triangle::GPUTriangle;
use engine_bvh::bvh::BVH;
use std::time::Instant;

type RenderSphere = engine_config::Sphere;
type RenderUniforms = engine_config::Uniforms;
type RenderMesh = engine_config::Mesh;
pub type RenderCamera = engine_config::Camera;
type RenderLight = engine_config::PointLight;
type RenderGeometry = (Vec<f32>, Vec<u32>, Vec<f32>, engine_config::Material);
type SubMeshGeometry = (Vec<f32>, Vec<u32>, Vec<f32>);

fn light_to_render_point_light(light: &LightSource) -> Option<RenderLight> {
    //! Converts the given LightSource to a engine_config::PointLight if has the type Point
    //! ## Parameter:
    //! 'light': LightSource that is to be converted
    //! ## Returns
    //! Options of engine_config::PointLight: Some if light has Type Point
    match light.get_light_type() {
        scene_objects::light_source::LightType::Point => Some(RenderLight::new(
            light.get_position().into(),
            0.5,
            light.get_luminositoy(),
            light.get_color(),
        )),
        _ => None,
    }
}

fn sphere_to_render_sphere(sphere: &Sphere) -> RenderSphere {
    //! Converts a given scene_objects::sphere::Sphere to a engine_config::sphere
    //! so it can be passed to the render engine
    //! ## Parameter
    //! scene_objects::sphere::Sphere to be converted
    //! ## Returns
    //! engine_config::Sphere based on the given sphere
    let center = sphere.get_center();
    let color = sphere.get_color();

    RenderSphere::new(
        engine_config::Vec3::new(center.x, center.y, center.z),
        sphere.get_radius(),
        engine_config::Material {
            diffuse: engine_config::Vec3::new(0.0, 0.0, 0.0),
            specular: [color[0], color[1], color[2]],
            ..Default::default()
        },
    )
    .unwrap()
    //todo error handling
}

fn vec3_to_array(vec: Vec3) -> [f32; 3] {
    [vec.x, vec.y, vec.z]
}
fn camera_to_render_uniforms(
    camera: &Camera,
    spheres_count: u32,
    color_hash_enabled: bool,
    bvh_node_count: u32,
    bvh_triangle_count: u32,
) -> Result<RenderUniforms, Error> {
    //! converts the given scene_object::camera::Camera to a render_config::Uniforms
    //! so that it can be passed to the render engine
    //! ## Parameter
    //! 'camera': scene_object::camer::Camera to be converted <br>
    //! 'spheres_count': Number of spheres to be rendered <br>
    //! 'triangles_count': Number of triangles to be rendered <br>
    //! 'color_hash_enabled': Whether color hash is enabled
    //! ## Returns
    //! render_config::Unfiforms for the given parameters
    let position = camera.get_position();
    let dir = camera.get_look_at() - camera.get_position(); //Engine uses currently a direction vector
    let render_camera = RenderCamera::new(
        camera.get_pane_distance(),
        camera.get_pane_width(),
        vec3_to_array(position),
        vec3_to_array(dir),
    );

    let Resolution { width, height } = camera.get_resolution();
    let uniforms = RenderUniforms::new(
        *width,
        *height,
        render_camera,
        camera.get_ray_samples(), //samples: ray per pixel
        spheres_count,
        bvh_node_count,
        bvh_triangle_count,
    )
    .with_color_hash(color_hash_enabled);
    Ok(uniforms)
}
fn material_to_render_material(
    mat: &scene_objects::material::Material,
    texture_map: &HashMap<String, i32>,
) -> engine_config::Material {
    let ambient = [
        mat.ambient_reflectivity[0] as f32,
        mat.ambient_reflectivity[1] as f32,
        mat.ambient_reflectivity[2] as f32,
    ];
    let diffuse = engine_config::Vec3::new(
        mat.diffuse_reflectivity[0] as f32,
        mat.diffuse_reflectivity[1] as f32,
        mat.diffuse_reflectivity[2] as f32,
    );
    let specular = [
        mat.specular_reflectivity.first().copied().unwrap_or(0.0) as f32,
        mat.specular_reflectivity.get(1).copied().unwrap_or(0.0) as f32,
        mat.specular_reflectivity.get(2).copied().unwrap_or(0.0) as f32,
    ];

    let texture_index = if let Some(path) = &mat.texture_path {
        *texture_map.get(path).unwrap_or(&-1)
    } else {
        -1
    };

    engine_config::Material::new(
        ambient,
        diffuse,
        specular,
        mat.shininess as f32,
        [0.0, 0.0, 0.0],               // emissive
        1.0,                           // ior
        1.0 - mat.transparency as f32, // opacity
        2,                             // illum (default to specular)
        texture_index,
    )
    .unwrap_or_default()
}

fn mesh_to_render_data(mesh: &Mesh, texture_map: &HashMap<String, i32>) -> Vec<RenderGeometry> {
    //! Extracts vertices and point references from the given mesh
    //! ## Parameter
    //! 'mesh': Mesh from scene_objects crate that is to be converted
    //! Returns: Vector of tuples: (vertices, indices, uvs, material)

    let original_vertices = mesh.get_vertices();
    let original_indices = mesh.get_tri_indices();
    let original_uvs = mesh.get_uvs();

    let materials = mesh.get_materials();
    let material_indices = mesh.get_material_indices();

    if let (Some(mats), Some(mat_indices)) = (materials, material_indices)
        && !mats.is_empty()
        && !mat_indices.is_empty()
    {
        let mut sub_meshes: HashMap<usize, SubMeshGeometry> = HashMap::new();

        let num_triangles = original_indices.len() / 3;

        for i in 0..num_triangles {
            let mat_idx = if i < mat_indices.len() {
                mat_indices[i]
            } else {
                0
            };

            let entry = sub_meshes
                .entry(mat_idx)
                .or_insert((Vec::new(), Vec::new(), Vec::new()));
            let (verts, inds, uvs) = entry;

            let current_v_count = (verts.len() / 3) as u32;

            let idx0 = original_indices[i * 3] as usize;
            let idx1 = original_indices[i * 3 + 1] as usize;
            let idx2 = original_indices[i * 3 + 2] as usize;

            // Add vertices
            verts.push(original_vertices[idx0 * 3]);
            verts.push(original_vertices[idx0 * 3 + 1]);
            verts.push(original_vertices[idx0 * 3 + 2]);

            verts.push(original_vertices[idx1 * 3]);
            verts.push(original_vertices[idx1 * 3 + 1]);
            verts.push(original_vertices[idx1 * 3 + 2]);

            verts.push(original_vertices[idx2 * 3]);
            verts.push(original_vertices[idx2 * 3 + 1]);
            verts.push(original_vertices[idx2 * 3 + 2]);

            // Add UVs
            if let Some(orig_uvs) = original_uvs {
                if idx0 * 2 + 1 < orig_uvs.len() {
                    uvs.push(orig_uvs[idx0 * 2]);
                    uvs.push(orig_uvs[idx0 * 2 + 1]);
                } else {
                    uvs.push(0.0);
                    uvs.push(0.0);
                }

                if idx1 * 2 + 1 < orig_uvs.len() {
                    uvs.push(orig_uvs[idx1 * 2]);
                    uvs.push(orig_uvs[idx1 * 2 + 1]);
                } else {
                    uvs.push(0.0);
                    uvs.push(0.0);
                }

                if idx2 * 2 + 1 < orig_uvs.len() {
                    uvs.push(orig_uvs[idx2 * 2]);
                    uvs.push(orig_uvs[idx2 * 2 + 1]);
                } else {
                    uvs.push(0.0);
                    uvs.push(0.0);
                }
            } else {
                for _ in 0..6 {
                    uvs.push(0.0);
                }
            }

            // Add indices
            inds.push(current_v_count);
            inds.push(current_v_count + 1);
            inds.push(current_v_count + 2);
        }

        let mut result = Vec::new();
        for (mat_idx, (verts, inds, uvs)) in sub_meshes {
            let material = if mat_idx < mats.len() {
                material_to_render_material(&mats[mat_idx], texture_map)
            } else {
                engine_config::Material::default()
            };
            result.push((verts, inds, uvs, material));
        }
        return result;
    }

    let vertices = original_vertices.clone();
    let indices = original_indices.clone();
    let uvs = if let Some(uvs) = original_uvs {
        uvs.clone()
    } else {
        vec![0.0; (vertices.len() / 3) * 2]
    };

    let material = if let Some(mats) = materials {
        if !mats.is_empty() {
            material_to_render_material(&mats[0], texture_map)
        } else {
            engine_config::Material::default()
        }
    } else {
        engine_config::Material::default()
    };

    vec![(vertices, indices, uvs, material)]
}

fn mesh_to_gpu_triangles(
    mesh: &RenderMesh,
    verts: &[f32],
    indices: &[u32],
    mesh_index: u32,
) -> Vec<GPUTriangle> {
    let start = (mesh.triangle_index_start * 3) as usize;
    let end = ((mesh.triangle_index_start + mesh.triangle_count) * 3) as usize;

    let mut tris = Vec::with_capacity(mesh.triangle_count as usize);

    for i in (start..end).step_by(3) {
        let v0i = indices[i] as usize;
        let v1i = indices[i + 1] as usize;
        let v2i = indices[i + 2] as usize;

        let i0 = v0i * 3;
        let i1 = v1i * 3;
        let i2 = v2i * 3;

        tris.push(GPUTriangle {
            v0: Vec3::new(verts[i0], verts[i0 + 1], verts[i0 + 2]),
            v1: Vec3::new(verts[i1], verts[i1 + 1], verts[i1 + 2]),
            v2: Vec3::new(verts[i2], verts[i2 + 1], verts[i2 + 2]),
            v0_index: v0i as u32,
            v1_index: v1i as u32,
            v2_index: v2i as u32,
            mesh_index,
            _pad0: 0,
            _pad1: 0,
            _pad2: 0,
        });
    }
    tris
}

/// Extends scene to offer functionalities needed for rendering with raytracer or pathtracer engine
impl Scene {
    fn get_render_point_lights(&self) -> Vec<RenderLight> {
        //! ## Returns
        //! A vector with all engine_config::PointLight from self
        let mut res = vec![];
        for light in self.get_light_sources() {
            if let Some(render_light) = light_to_render_point_light(light) {
                res.push(render_light);
            }
        }
        res
    }
    fn get_render_spheres(&self) -> Vec<RenderSphere> {
        //! ## Returns
        //! a Vec that contains all Scene spheres as engine_config::Sphere
        self.get_spheres()
            .iter()
            .map(sphere_to_render_sphere)
            .collect()
    }
    pub(crate) fn get_render_uniforms(
        &self,
        spheres_count: u32,
        bvh_node_count: u32,
        bvh_triangle_count: u32,
    ) -> RenderUniforms {
        //! ## Returns
        //! RenderUnfiform for the camera of the scene
        camera_to_render_uniforms(
            self.get_camera(),
            spheres_count,
            self.get_color_hash_enabled(),
            bvh_node_count,
            bvh_triangle_count,
        )
        .unwrap()
    }

    fn get_render_tris(&self, texture_map: &HashMap<String, i32>) -> Vec<RenderGeometry> {
        //! ## Returns
        //! Vector of touples, with each of the touples representing a TriGeometry defined by the points and the triangles build from the points.
        self.get_meshes()
            .iter()
            .flat_map(|m| mesh_to_render_data(m, texture_map))
            .collect()
    }

    pub fn render(&mut self) -> Result<RenderOutput, Error> {
        //! calls the render engine for the scene self.
        //! ## Returns
        //! Result of either the RenderOutput or a error
        info!("{self}: Render has been called. Collecting render parameters");
        let start = Instant::now();

        let render_spheres = self.get_render_spheres();

        // Collect textures
        let mut texture_list = Vec::new();
        let mut texture_map = HashMap::new();

        for (path, data) in &self.textures {
            texture_map.insert(path.clone(), texture_list.len() as i32);
            texture_list.push(data.clone());
        }

        let render_tris = self.get_render_tris(&texture_map);
        debug!("Scene mesh data: {:?}", self.get_meshes());
        debug!("Collected mesh data: {:?}", render_tris);

        let spheres_count = render_spheres.len() as u32;

        // Collect all vertices, triangles, and mesh into flat vectors
        let (all_vertices, all_triangles, all_meshes, all_uvs) = if render_tris.is_empty() {
            (vec![], vec![], vec![], vec![])
        } else {
            let mut all_verts = vec![];
            let mut all_tris = vec![];
            let mut all_uvs = vec![];
            let mut mesh_infos = vec![];
            let mut vertex_offset = 0u32;
            let mut triangle_offset = 0u32;

            for (verts, tris, uvs, material) in render_tris.iter() {
                let vertex_count = (verts.len() / 3) as u32;
                let triangle_count = (tris.len() / 3) as u32;

                // Add mesh metadata
                mesh_infos.push(RenderMesh::new(triangle_offset, triangle_count, *material));

                // Add triangles with vertex offset
                for tri_idx in tris {
                    all_tris.push(tri_idx + vertex_offset);
                }

                // Add vertices
                all_verts.extend(verts);

                // Add UVs
                all_uvs.extend(uvs);

                vertex_offset += vertex_count;
                triangle_offset += triangle_count;
            }

            (all_verts, all_tris, mesh_infos, all_uvs)
        };

        let mut gpu_triangles: Vec<GPUTriangle> = Vec::new();

        for (i, mesh) in all_meshes.iter().enumerate() {
            gpu_triangles.extend(mesh_to_gpu_triangles(
                mesh,
                &all_vertices,
                &all_triangles,
                i as u32,
            ));
        }

        let (bvh_nodes, bvh_indices) = if gpu_triangles.is_empty() {
            (vec![], vec![])
        } else {
            let bvh = BVH::new(&gpu_triangles);
            (bvh.nodes, bvh.indices)
        };

        let bvh_node_count = bvh_nodes.len();
        let bvh_triangle_count = gpu_triangles.len();

        let uniforms = self.get_render_uniforms(
            spheres_count,
            bvh_node_count as u32,
            bvh_triangle_count as u32,
        );

        info!("Collected vertices count: {}", all_vertices.len());
        info!("Collected tris count: {}", all_triangles.len());
        info!(
            "{self}: Collected render parameter: {} spheres, {} triangles consisting of {} vertices. Building render config",
            render_spheres.len(),
            bvh_triangle_count,
            all_vertices.len() / 3
        );

        let point_lights = self.get_render_point_lights();

        let rc = if self.get_first_render() {
            self.set_first_render(false);
            // NOTE: *_create is for the first initial render which initializes all the buffers etc.
            RenderConfigBuilder::new()
                .uniforms_create(uniforms)
                .spheres_create(render_spheres)
                .uvs_create(all_uvs)
                .meshes_create(all_meshes)
                .bvh_nodes_create(bvh_nodes)
                .bvh_indices_create(bvh_indices)
                .bvh_triangles_create(gpu_triangles)
                .lights_create(point_lights)
                .textures_create(texture_list)
                .build()
        } else {
            // NOTE: * otherwise the values are updated with the new value an the unchanged fields
            // are kept as is. See: ../../../crates/engine-config/src/render_config.rs - `Change<T>`
            RenderConfigBuilder::new()
                .uniforms(uniforms)
                .spheres(render_spheres)
                .uvs(all_uvs)
                .meshes(all_meshes)
                .bvh_nodes_create(bvh_nodes)
                .bvh_indices_create(bvh_indices)
                .bvh_triangles_create(gpu_triangles)
                .lights(point_lights)
                .textures(texture_list)
                .build()
        };

        let engine = self.get_render_engine_mut();

        let output = engine.render(rc);
        let duration = start.elapsed();
        info!("Execution Time: {:?}", duration);
        match output {
            Ok(res) => match res.validate() {
                Ok(_) => {
                    info!("{self}: Successfully got valid render output");
                    self.set_last_render(res.clone());
                    Ok(res)
                }
                Err(error) => {
                    error!("{self}: Received invalid render output");
                    Err(error)
                }
            },
            Err(error) => {
                error!("{self}: The following error occurred when rendering: {error}");
                Err(error)
            }
        }
    }
}
