use anyhow::Result;
use eframe::egui;
use engine_config::{Camera, RenderConfig, RenderConfigBuilder, RenderEngine, Sphere};
use engine_main::Engine;
use engine_core::RenderOutput;

/* START TEMPORARY EXAMPLE CODE - THIS SHOULD BE MOVED INTO ITS OWN CRATE(S) */
static WIDTH: usize = 1920 / 2;
static HEIGHT: usize = 1080 / 2;
static FOV: f32 = std::f32::consts::FRAC_PI_4;

const SPHERES: [Sphere; 5] = [
    Sphere {
        center: [0.0, 0.6, 1.0],
        radius: 0.5,
        color: [1.0, 0.0, 1.0],
        _pad: [0u8; 4],
    }, // Top, magenta
    Sphere {
        center: [-0.6, 0.0, 1.0],
        radius: 0.5,
        color: [0.0, 1.0, 0.0],
        _pad: [0u8; 4],
    }, // Left, green
    Sphere {
        center: [0.0, 0.0, 1.0],
        radius: 0.5,
        color: [1.0, 0.0, 0.0],
        _pad: [0u8; 4],
    }, // Centered, red
    Sphere {
        center: [0.6, 0.0, 1.0],
        radius: 0.5,
        color: [0.0, 0.0, 1.0],
        _pad: [0u8; 4],
    }, // Right, blue
    Sphere {
        center: [0.0, -0.6, 1.0],
        radius: 0.5,
        color: [0.0, 1.0, 1.0],
        _pad: [0u8; 4],
    }, // Bottom, cyan
];

pub struct App {
    image: Option<egui::TextureHandle>,
    dirty: bool,
    renderer: Option<Engine>,
    fov: f32,
    width: usize,
    height: usize,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let camera = Camera::new(WIDTH as u32, HEIGHT as u32, 1.0).unwrap();
        let (builder, _) = match
            RenderConfigBuilder::new()
                .camera(camera)
                .unwrap()
                .spheres(SPHERES.into())
                .unwrap()
                .engine(RenderEngine::Raytracer)
                .build()
         {
            Ok(r) => (Some(r), None),
            Err(e) => {
                let msg = format!("Renderer initialization failed: {}", e);
                (None, Some(msg))
            }
        };

        let renderer = Engine::new(builder.unwrap());

        Self {
            image: None,
            dirty: true,
            renderer: Some(renderer),
            fov: FOV,
            width: WIDTH,
            height: HEIGHT,
        }
    }

    pub fn update_image_from_output(&mut self, ctx: &egui::Context, output: &RenderOutput) {
        let size = [output.width, output.height];
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &output.pixels);
        self.image = Some(ctx.load_texture("output", color_image, egui::TextureOptions::LINEAR));
    }

    fn make_updated_render_config(&mut self) -> Result<RenderConfig> {
        let camera = Camera::new((self.width) as u32, (self.height) as u32, self.fov).unwrap();
        RenderConfigBuilder::new()
            .camera(camera)?
            .spheres(SPHERES.into())?
            .engine(RenderEngine::Raytracer)
            .build()
    }

    fn update_render(&mut self, ctx: &egui::Context) {
        if self.renderer.is_none() {
            self.dirty = false;
            return;
        }

        let rc = match self.make_updated_render_config() {
            Ok(rc) => rc,
            Err(e) => {
                log::error!("RenderConfigBuilding failed: {}", e);
                self.dirty = false;
                return;
            }
        };

        let output = match self.renderer.as_mut().unwrap().render(rc) {
            Ok(output) => output,
            Err(e) => {
                log::error!("Render failed: {}", e);
                self.dirty = false;
                return;
            }
        };

        if let Err(e) = output.validate() {
            log::error!("Invalid render output: {}", e);
            self.dirty = false;
            return;
        }

        self.update_image_from_output(ctx, &output);
        self.dirty = false;
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rendered Output");

            if ui.button("Render").clicked() || self.dirty {
                self.update_render(ctx);
            }

            if ui
                .add(
                    egui::Slider::new(&mut self.fov, Camera::MIN_FOV..=Camera::MAX_FOV).text("FOV"),
                )
                .changed()
            {
                self.update_render(ctx);
            }

            if ui
                .add(
                    egui::Slider::new(&mut self.width, 1..=2000).text("WIDTH"),
                )
                .changed()
            {
                self.update_render(ctx);
            }

            if ui
                .add(
                    egui::Slider::new(&mut self.height, 1..=2000).text("HEIGHT"),
                )
                .changed()
            {
                self.update_render(ctx);
            }

            if let Some(img) = &self.image {
                ui.image((img.id(), img.size_vec2()));
            } else {
                ui.label("Click 'Render' to generate the image");
            }
        });
    }
}

/* END TEMPORARY EXAMPLE CODE */

const EGUI_DEFAULT_WINDOW_DIMENSION: (f32, f32) = (1280.0, 720.0);

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([
                EGUI_DEFAULT_WINDOW_DIMENSION.0,
                EGUI_DEFAULT_WINDOW_DIMENSION.1,
            ])
            .with_title("RenderBaby Playground"),
        ..Default::default()
    };

    eframe::run_native(
        "RenderBaby Playground",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
