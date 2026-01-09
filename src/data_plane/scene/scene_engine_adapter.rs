/// Serves as an adpter between the scene plane and the render engine.
use std::collections::HashMap;
use anyhow::{Error, Result};
use glam::Vec3;
use scene_objects::{
    camera::{Camera, Resolution},
    light_source::LightSource,
    mesh::Mesh,
    sphere::Sphere,
};

type RenderSphere = engine_config::Sphere;
type RenderUniforms = engine_config::Uniforms;
type RenderMesh = engine_config::Mesh;
pub type RenderCamera = engine_config::Camera;
type RenderLight = engine_config::PointLight;
type RenderGeometry = (Vec<f32>, Vec<u32>, Vec<f32>, engine_config::Material);
type SubMeshGeometry = (Vec<f32>, Vec<u32>, Vec<f32>);

pub(super) fn light_to_render_point_light(light: &LightSource) -> Option<RenderLight> {
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

pub(super) fn sphere_to_render_sphere(sphere: &Sphere) -> RenderSphere {
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
pub(super) fn camera_to_render_uniforms(
    camera: &Camera,
    spheres_count: u32,
    triangles_count: u32,
    color_hash_enabled: bool,
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
        triangles_count,
    )
    .with_color_hash(color_hash_enabled);
    Ok(uniforms)
}
pub(super) fn material_to_render_material(
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
        mat.specular_reflectivity[0] as f32,
        mat.specular_reflectivity[1] as f32,
        mat.specular_reflectivity[2] as f32,
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

pub(super) fn mesh_to_render_data(
    mesh: &Mesh,
    texture_map: &HashMap<String, i32>,
) -> Vec<RenderGeometry> {
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
