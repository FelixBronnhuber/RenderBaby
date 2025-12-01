mod compute_plane;
mod control_plane;
mod data_plane;

use std::env;
use crate::control_plane::cli::CliApp;
use crate::control_plane::gui::app::App;

fn main() {
    /* for testing without setting env vars yourself: */
    if env::var("RUST_LOG").is_err() {
        unsafe { env::set_var("RUST_LOG", "debug") }
    }
    log_buffer::get_builder().init();

    if let Some(cli) = CliApp::parse() {
        cli.run();
    } else {
        let app = App::new();
        app.show();
    }
}
