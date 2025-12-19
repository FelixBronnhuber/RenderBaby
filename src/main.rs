mod compute_plane;
mod control_plane;
mod data_plane;

use std::env;
use std::path::PathBuf;
use control_plane::modes;
use crate::data_plane::scene::render_scene::Scene;

fn main() {
    /*let mut path1 = PathBuf::new();
        path1.push("C:/Users/fucjo/RustroverProjects/RenderBaby/fixtures/scenes/scene.json");

    let scene = Scene::load_scene_from_file(path1).unwrap();
    println!("{}, \n{}, \n{:?}, \n{:?}, \n{:?}", scene.get_name(),scene.get_camera(),scene.get_meshes(),scene.get_light_sources(),scene.get_background_color());

    */

    /* for testing without setting env vars yourself: */
    if env::var("RUST_LOG").is_err() {
        unsafe { env::set_var("RUST_LOG", "info") }
    }
    log_buffer::get_builder().init();

    let app = modes::get_app();
    app.show();
}
