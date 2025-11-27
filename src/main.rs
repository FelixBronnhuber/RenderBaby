use control_plane::App;

fn main() {
    env_logger::init();

    let app = App::new();
    app.show();
}
