mod compute_plane;
mod control_plane;
mod data_plane;

use crate::control_plane::gui::app::App;

fn main() {
    env_logger::init();

    let app = App::new();
    app.show();
}
