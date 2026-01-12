use crate::control_plane::app::App;

pub struct BenchmarkApp {}

impl App for BenchmarkApp {
    fn new() -> Self {
        Self {}
    }

    fn show(self: Box<BenchmarkApp>) {
        todo!("Do the actual implementation of running the app here")

        // Orientiere dich an der cli_static.rs
        // e.g.
        // let scene = Scene::load_scene_from_path(PathBuf::from("some fixture"), true).unwrap();
        // let time1 = std::time::Instant::now();
        // let _ = scene.render();
        // let time2 = std::time::Instant::now();
        // println!("Render time: {:?}", time2.duration_since(time1));
    }
}
