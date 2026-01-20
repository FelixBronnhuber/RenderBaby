mod compute_plane;
mod control_plane;
mod data_plane;
mod included_files;

use control_plane::modes;

/// Runs the application.
///
/// Set the log-level manually with RUST_LOG:
/// RUST_LOG=debug|info|warn|error
fn main() {
    // default is info
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", "info");
        }
    }

    log_buffer::get_builder().init();

    let app = modes::get_app();
    app.show();
}
