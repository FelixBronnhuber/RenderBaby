use std::time::Instant;
use log::{info};
use sysinfo::{System};
use wgpu::Instance;
use crate::control_plane::app::App;
use crate::data_plane::scene::render_scene::Scene;

const SAMPLE_COUNTS: &[u32] = &[1, 100,];
pub struct BenchmarkApp;

impl BenchmarkApp {
    pub fn new() -> Self {
        Self {}
    }
    fn benchmark(sample_count: u32) -> std::time::Duration {
        let mut scene =
            Scene::load_scene_from_path("fixtures/scenes/scene.json".parse().unwrap(), true)
                .unwrap();
        scene.render().expect("Render failed");
        scene.get_camera_mut().set_ray_samples(sample_count);
        scene.set_color_hash_enabled(false);
        let start = Instant::now();
        scene.render().expect("Render failed");
        start.elapsed()
    }
}

impl App for BenchmarkApp {
    fn show(self: Box<BenchmarkApp>) {
        let mut results: Vec<(u32, std::time::Duration)> = Vec::new();

        for &samples in SAMPLE_COUNTS {
            info!("Running render with {} samples...", samples);

            let duration = Self::benchmark(samples);
            results.push((samples, duration));
        }

        let mut sys = System::new_all();
        sys.refresh_all();
        let instance = Instance::default();
        let adapter = pollster::block_on(
            instance.request_adapter(&wgpu::RequestAdapterOptions::default())
        ).unwrap();

        info!("----------------------------");
        info!("Hardware Specs:");

        let gpu = adapter.get_info();
        info!("GPU: {}", gpu.name);
        info!("Backend: {:?}", gpu.backend);

        info!("CPU: {}", sys.cpus()[0].brand());
        info!("Cores: {}", sys.cpus().len());

        info!("Total RAM: {} MB", sys.total_memory() / 1024);
        info!("Available RAM: {} MB", sys.available_memory() / 1024);

        if let Some(os) = System::long_os_version() {
            info!("OS: {}", os);
        } else {
            info!("OS: Unknown");
        }
        if let Some(ke) = System::kernel_version() {
            info!("Kernel: {}", ke);
        } else {
            info!("Kernel: Unknown");
        }
        info!("Architecture: {}", std::env::consts::ARCH);

        info!("----------------------------");
        info!("Benchmark results:");

        info!("{:>10} | {:>15}", "Samples", "Render Time");
        info!("-----------+----------------");

        for (samples, duration) in results {
            info!("{:>10} | {:>15.3?}", samples, duration);
        }
        info!("----------------------------");
    }
}
