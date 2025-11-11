use eframe::egui;
use engine_wgpu_wrapper::{EngineType, RenderOutput, WgpuWrapper};
use scene::scene::Scene;
/* START TEMPORARY EXAMPLE CODE - THIS SHOULD BE MOVED INTO ITS OWN CRATE */
static WIDTH: usize = 1920 / 2;
static HEIGHT: usize = 1080 / 2;

pub struct App {
    image: Option<egui::TextureHandle>,
    dirty: bool,
    renderer: Option<WgpuWrapper>,
    scene: Scene
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (renderer, _) = match WgpuWrapper::new(EngineType::Raytracer, WIDTH, HEIGHT) {
            Ok(r) => (Some(r), None),
            Err(e) => {
                let msg = format!("Renderer initialization failed: {}", e);
                (None, Some(msg))
            }
        };
        let mut scene = Scene::new();
        scene.proto_init();
        Self {
            image: None,
            dirty: true,
            renderer,
            scene
        }
    }

    pub fn update_image_from_output(&mut self, ctx: &egui::Context, output: &RenderOutput) {
        let size = [output.width, output.height];
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &output.pixels);
        self.image = Some(ctx.load_texture("output", color_image, egui::TextureOptions::LINEAR));
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rendered Output");

/*             if ui.button("Render").clicked() || self.dirty {
                if let Some(scene) = &mut self.scene {
                    match scene.render() {
                        Ok(output) => match output.validate() {
                            Err(e) => {
                                log::error!("Invalid render output: {}", e);
                            }
                            Ok(_) => {
                                self.update_image_from_output(ctx, &output);
                            }
                        },
                        Err(e) => log::error!("Render failed: {}", e),
                    }
                }
                self.dirty = false;
            } */
            if ui.button("Render").clicked() || self.dirty {
                let render = self.scene.render();
                match render {
                    Ok(output) => match output.validate() {
                        Err(e) => {
                            log::error!("Invalid render output: {}", e);
                        }
                        Ok(_) => {
                            self.update_image_from_output(ctx, &output)
                        }
                    },
                    Err(e) => log::error!("Render failed: {}", e),
                }
                //self.update_image_from_output(ctx, &output);
                //self.dirty = false;
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
