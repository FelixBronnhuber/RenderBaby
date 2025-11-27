mod mvc;

fn main() {
    env_logger::init();

    let app = mvc::App::new();
    app.show();
}
