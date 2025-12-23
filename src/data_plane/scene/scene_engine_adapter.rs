/// Serves as an adpter between the scene plane and the render engine.
use anyhow::{Error, Result};
use glam::Vec3;
use scene_objects::{
    camera::{Camera, Resolution},
    mesh::Mesh,
    sphere::Sphere,
};

type RenderSphere = engine_config::Sphere;
type RenderUniforms = engine_config::Uniforms;
type RenderCamera = engine_config::Camera;

pub fn sphere_to_render_sphere(sphere: &Sphere) -> RenderSphere {
    //! Converts a given scene_objects::sphere::Sphere to a engine_config::sphere
    //! so it can be passed to the render engine
    //! ## Parameter
    //! scene_objects::sphere::Sphere to be converted
    //! ## Returns
    //! engine_config::Sphere based on the given sphere
    RenderSphere::new(
        {
            let center = sphere.get_center();
            engine_config::Vec3::new(center.x, center.y, center.z)
        },
        sphere.get_radius(),
        {
            let color = sphere.get_color();
            engine_config::Vec3::new(color[0], color[1], color[2])
        },
    )
    .unwrap()
    //todo error handling
}
fn vec3_to_array(vec: Vec3) -> [f32; 3] {
    [vec.x, vec.y, vec.z]
}
pub fn camera_to_render_uniforms(
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
pub fn mesh_to_render_data(mesh: &Mesh) -> (Vec<f32>, Vec<u32>) {
    //! Extracts vertices and point references from the given mesh
    //! ## Parameter
    //! 'mesh': Mesh from scene_objects crate that is to be converted
    //! Returns: touple of: Vec<f32> where 3 entries define one point in 3d space, and Vec<u32> referencing which points make up a triangle
    (mesh.get_vertices().clone(), mesh.get_tri_indices().clone())
}
