use eframe::{
    egui::{self, Color32, RichText,ColorImage},
};

//Mockup enum for Categories of Objects
#[derive(PartialEq, Clone)]
enum Category {
    Camera,
    Lights,
    Objects,
}

//Mockup struct for Objects in a Scene
#[derive(Clone)]
struct SceneItem {
    name: String,
    category: Category,
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    pitch: f32,
    yaw: f32,
}

pub struct RaytracerApp {
    texture: Option<egui::TextureHandle>,
    dark_mode: bool,
    expanded_camera: bool,
    expanded_lights: bool,
    expanded_objects: bool,
    render_height: u32,
    render_width: u32,
    items: Vec<SceneItem>,
    selected: Option<usize>,
}

impl Default for RaytracerApp {
    fn default() -> Self {
        Self {
            texture: None,
            dark_mode: false,
            expanded_camera: true,
            expanded_lights: true,
            expanded_objects: true,
            render_height: 1920,
            render_width: 1080,
            items: vec![        //Initialize with example objects
                SceneItem {
                    name: "Main Camera".into(),
                    category: Category::Camera,
                    pos_x: 0.0,
                    pos_y: 1.0,
                    pos_z: 5.0,
                    pitch: 0.0,
                    yaw: 0.0,
                },
                SceneItem {
                    name: "Light 1".into(),
                    category: Category::Lights,
                    pos_x: 2.0,
                    pos_y: 3.0,
                    pos_z: -1.0,
                    pitch: 0.0,
                    yaw: 0.0,
                },
                SceneItem {
                    name: "Sphere 1".into(),
                    category: Category::Objects,
                    pos_x: 0.0,
                    pos_y: 0.0,
                    pos_z: 0.0,
                    pitch: 0.0,
                    yaw: 0.0,
                },
            ],
            selected: None,
        }
    }
}

impl eframe::App for RaytracerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Dark/Light Mode Switch
        ctx.set_visuals(if self.dark_mode {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        });

        //Initialize TopPanel for Options
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Import").clicked() {
                        ui.close_menu();
                    }
                    if ui.button("Export").clicked() {
                        ui.close_menu();
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("🌗 Dark/Light").clicked() {
                        self.dark_mode = !self.dark_mode;
                    }
                });
            });
        });

        //Initializing LeftPanel for SceneExplorer, Resolution and temporary RenderButton
        egui::SidePanel::left("explorer")
            .resizable(true)
            .min_width(220.0)
            .show(ctx, |ui| {
                ui.heading("Scene Explorer");
                ui.separator();

                let mut draw_category = |name: &str,
                                         cat: Category,
                                         expanded: &mut bool,
                                         items: &Vec<SceneItem>,
                                         selected: &mut Option<usize>,
                                         ui: &mut egui::Ui| {
                    egui::collapsing_header::CollapsingState::load_with_default_open(
                        ui.ctx(),
                        ui.make_persistent_id(name),
                        *expanded,
                    )
                        .show_header(ui, |ui| {
                            ui.label(RichText::new(name).strong());
                        })
                        .body(|ui| {
                            for (i, item) in items.iter().enumerate() {
                                if item.category == cat {
                                    let selected_now = *selected == Some(i);
                                    if ui.selectable_label(selected_now, &item.name).clicked() {
                                        *selected = Some(i);
                                    }
                                }
                            }
                        });
                };

                draw_category(
                    "Camera",
                    Category::Camera,
                    &mut self.expanded_camera,
                    &self.items,
                    &mut self.selected,
                    ui,
                );

                draw_category(
                    "Lights",
                    Category::Lights,
                    &mut self.expanded_lights,
                    &self.items,
                    &mut self.selected,
                    ui,
                );

                draw_category(
                    "Objects",
                    Category::Objects,
                    &mut self.expanded_objects,
                    &self.items,
                    &mut self.selected,
                    ui,
                );
                ui.separator();
                ui.vertical(|ui| {
                    ui.heading("Resolution");
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Height:");
                        ui.add(egui::DragValue::new(&mut self.render_height).speed(1.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Width:");
                        ui.add(egui::DragValue::new(&mut self.render_width).speed(1.0));
                    });
                });

                ui.vertical_centered(|ui| {
                    if ui.add_sized([180.0, 40.0], egui::Button::new("▶ Render")).clicked() {
                        // Here the renderer should be started
                        println!("Render button clicked!");

                        //example image
                        let width = 512;
                        let height = 512;
                        let mut test_image = vec![0u8; width * height * 4];
                        for y in 0..height {
                            for x in 0..width {
                                let i = (y * width + x) * 4;
                                test_image[i] = (x as u8);
                                test_image[i + 1] = (y as u8);
                                test_image[i + 2] = 128;
                                test_image[i + 3] = 255;
                            }
                        }

                        let color_image = ColorImage::from_rgba_unmultiplied([width as usize, height as usize], &test_image);
                        self.texture = Some(ui.ctx().load_texture("image", color_image, egui::TextureOptions::NEAREST,));
                    }
                });
            });

        //Initializing BottomPanel to configure SceneItems
        egui::TopBottomPanel::bottom("properties_panel")
            .resizable(true)
            .default_height(200.0)
            .show(ctx, |ui| {
                ui.heading("Properties");
                ui.separator();

                if let Some(index) = self.selected {
                    let item = &mut self.items[index];
                    ui.label(RichText::new(&item.name).strong());
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Position:");
                        ui.add(egui::DragValue::new(&mut item.pos_x).speed(0.1).prefix("x: "));
                        ui.add(egui::DragValue::new(&mut item.pos_y).speed(0.1).prefix("y: "));
                        ui.add(egui::DragValue::new(&mut item.pos_z).speed(0.1).prefix("z: "));
                    });

                    // Rotation is not given for Lights
                    match item.category {
                        Category::Camera | Category::Objects => {
                            ui.add_space(10.0);
                            ui.label("Rotation:");
                            ui.add(
                                egui::Slider::new(&mut item.pitch, -90.0..=90.0)
                                    .text("Pitch")
                                    .clamp_to_range(true),
                            );
                            ui.add(
                                egui::Slider::new(&mut item.yaw, -180.0..=180.0)
                                    .text("Yaw")
                                    .clamp_to_range(true),
                            );
                            ui.add_space(1.0)
                        }
                        Category::Lights => {
                            ui.add_space(1.0);
                        }
                    }
                } else {
                    ui.label("Select an item from the explorer to edit its properties.");
                    ui.add_space(1.0)
                }
            });

        //Placeholder for Main Render Area
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {

                if let Some(texture) = &self.texture{

                    let aspect = texture.size_vec2().x / texture.size_vec2().y;

                    let size_scaled = if ui.available_size().x / ui.available_size().y > aspect {

                        egui::vec2(ui.available_size().y * aspect, ui.available_size().y)
                    } else {

                        egui::vec2(ui.available_size().x, ui.available_size().x / aspect)
                    };

                    ui.image((texture.id(), size_scaled));
                }else{
                    ui.label(
                        RichText::new("Render Output Area")
                            .color(Color32::from_rgb(80, 80, 80))
                            .size(20.0),
                    );
                }
            });
        });
    }
}

pub fn start() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "RenderBaby Raytracer",
        native_options,
        Box::new(|_cc| Ok(Box::<RaytracerApp>::default())),
    )
}
