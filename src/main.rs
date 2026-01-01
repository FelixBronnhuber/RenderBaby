mod compute_plane;
mod control_plane;
mod data_plane;

use std::env;

use control_plane::modes;
use crate::data_plane::scene::render_scene;

fn main() {
    /* for testing without setting env vars yourself: */
    if env::var("RUST_LOG").is_err() {
        unsafe { env::set_var("RUST_LOG", "info") }
    }
    log_buffer::get_builder().init();

    let app = modes::get_app();
    app.show();
}
