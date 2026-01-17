use std::sync::{Arc, Mutex};
use egui::{CollapsingHeader, Color32, RichText, Ui};
use scene_objects::camera::Resolution;
use scene_objects::geometric_object::GeometricObject;
use scene_objects::light_source::LightSource;
use scene_objects::material::Material;
use scene_objects::mesh::Mesh;
use scene_objects::sphere::Sphere;
use crate::data_plane::scene::render_scene::Scene;
use crate::data_plane::scene_proxy::color::Color;
use crate::data_plane::scene_proxy::misc::Misc;
use crate::data_plane::scene_proxy::position::Vec3d;
use crate::data_plane::scene_proxy::proxy_camera::ProxyCamera;
use crate::data_plane::scene_proxy::proxy_light::ProxyLight;
use crate::data_plane::scene_proxy::proxy_mesh::ProxyMesh;
use crate::data_plane::scene_proxy::proxy_sphere::ProxySphere;

pub trait Viewable {
    type RealSceneObject;
    fn ui(&mut self, ui: &mut Ui, object: &mut Self::RealSceneObject) -> bool;
}

fn vec3_ui(ui: &mut Ui, vec: &mut Vec3d) -> bool {
    let mut changed = false;

    ui.horizontal(|ui| {
        ui.label("X:");
        changed |= ui
            .add(
                egui::DragValue::new(&mut vec.x)
                    .speed(0.1)
                    .range(-1_000.0..=1_000.0),
            )
            .changed();
        ui.label("Y:");
        changed |= ui
            .add(
                egui::DragValue::new(&mut vec.y)
                    .speed(0.1)
                    .range(-1_000.0..=1_000.0),
            )
            .changed();
        ui.label("Z:");
        changed |= ui
            .add(
                egui::DragValue::new(&mut vec.z)
                    .speed(0.1)
                    .range(-1_000.0..=1_000.0),
            )
            .changed();
    });
    changed
}

fn color_ui(ui: &mut Ui, color: &mut Color) -> bool {
    let mut changed = false;

    ui.horizontal(|ui| {
        ui.label("R:");
        changed |= ui
            .add(
                egui::DragValue::new(&mut color.r)
                    .speed(0.01)
                    .range(0.0..=1.0),
            )
            .changed();
        ui.label("G:");
        changed |= ui
            .add(
                egui::DragValue::new(&mut color.g)
                    .speed(0.01)
                    .range(0.0..=1.0),
            )
            .changed();
        ui.label("B:");
        changed |= ui
            .add(
                egui::DragValue::new(&mut color.b)
                    .speed(0.01)
                    .range(0.0..=1.0),
            )
            .changed();
    });
    changed
}

impl Vec3d {
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Vec3d {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            Vec3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn cross(&self, other: &Vec3d) -> Vec3d {
        Vec3d {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn scale(&self, scalar: f32) -> Vec3d {
        Vec3d {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn add(&self, other: &Vec3d) -> Vec3d {
        Vec3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ProxyCamera {
    pub fn ui_with_settings(
        &mut self,
        ui: &mut Ui,
        scene: &mut Arc<Mutex<Scene>>,
        render_on_change: &mut bool,
    ) -> bool {
        let mut changed = false;

        ui.collapsing(
            RichText::new("ℹ Controls & Tips").color(Color32::LIGHT_BLUE),
            |ui| {
                ui.label(RichText::new("Viewfinding:").strong());
                ui.horizontal(|ui| {
                    ui.label("W/S: Forward/Backward");
                    ui.label("A/D: Left/Right");
                });
                ui.horizontal(|ui| {
                    ui.label("E/Q: Up/Down");
                });
                ui.label("Use 'Rener on change' for better viewfinding");
            },
        );

        ui.separator();

        if *render_on_change {
            ui.label(
                RichText::new("⚠ This feature affects Performance")
                    .color(Color32::ORANGE)
                    .strong(),
            );
            ui.label(RichText::new("Reduce samples/resolution for smoother navigation.").small());
        }
        ui.checkbox(render_on_change, "Render on change");

        ui.separator();

        // TODO: Get the fov limits from somewhere central as consts.
        if ui
            .add(egui::Slider::new(&mut self.pane_width, 0.1..=100.0).text("Pane Width"))
            .changed()
        {
            // TODO MICHAEL:
            scene
                .lock()
                .unwrap()
                .get_camera_mut()
                .set_pane_width(self.pane_width);
            changed = true;
        }

        if ui
            .add(egui::Slider::new(&mut self.pane_distance, 0.1..=100.0).text("Pane Distance"))
            .changed()
        {
            // TODO MICHAEL:
            scene
                .lock()
                .unwrap()
                .get_camera_mut()
                .set_pane_distance(self.pane_distance);
            changed = true;
        }

        ui.horizontal(|ui| {
            ui.label("Width:");
            if ui
                .add(
                    egui::DragValue::new(&mut self.resolution[0]), //.range(ImageResolution::MIN[0]..=ImageResolution::MAX[0]),
                )
                .changed()
            {
                // TODO MICHAEL:
                scene
                    .lock()
                    .unwrap()
                    .get_camera_mut()
                    .set_resolution(Resolution {
                        width: self.resolution[0],
                        height: self.resolution[1],
                    });
                changed = true;
            }
        });

        ui.horizontal(|ui| {
            ui.label("Height:");
            if ui
                .add(
                    egui::DragValue::new(&mut self.resolution[1]), //.range(ImageResolution::MIN[1]..=ImageResolution::MAX[1]),
                )
                .changed()
            {
                // TODO MICHAEL:
                scene
                    .lock()
                    .unwrap()
                    .get_camera_mut()
                    .set_resolution(Resolution {
                        width: self.resolution[0],
                        height: self.resolution[1],
                    });
                changed = true;
            }
        });
        ui.separator();

        ui.label("Camera Position:");
        if vec3_ui(ui, &mut self.position) {
            // TODO MICHAEL:
            scene
                .lock()
                .unwrap()
                .get_camera_mut()
                .set_position(self.position.clone().into());
            changed = true;
        }

        ui.label("Camera Direction:");
        if vec3_ui(ui, &mut self.look_at) {
            // TODO MICHAEL:
            scene
                .lock()
                .unwrap()
                .get_camera_mut()
                .set_look_at(self.look_at.clone().into());
            changed = true;
        }

        let delta_time = ui.input(|i| i.stable_dt);
        let speed = 5.0;
        let forward = Vec3d {
            x: self.look_at.x - self.position.x,
            y: self.look_at.y - self.position.y,
            z: self.look_at.z - self.position.z,
        }
        .normalize();
        let up = Vec3d {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let right = forward.cross(&up).normalize();
        let move_speed = speed * delta_time;
        let mut movement = Vec3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        // Check for keys
        if ui.input(|i| i.key_down(egui::Key::W)) {
            movement = movement.add(&forward.scale(move_speed));
        }
        if ui.input(|i| i.key_down(egui::Key::S)) {
            movement = movement.add(&forward.scale(-move_speed));
        }
        if ui.input(|i| i.key_down(egui::Key::A)) {
            movement = movement.add(&right.scale(move_speed));
        }
        if ui.input(|i| i.key_down(egui::Key::D)) {
            movement = movement.add(&right.scale(-move_speed));
        }
        if ui.input(|i| i.key_down(egui::Key::PageUp) || i.key_down(egui::Key::E)) {
            movement.y += move_speed;
        }
        if ui.input(|i| i.key_down(egui::Key::PageDown) || i.key_down(egui::Key::Q)) {
            movement.y -= move_speed;
        }

        // Apply movement if any key was pressed
        if movement.length() > 0.001 {
            self.position.x += movement.x;
            self.position.y += movement.y;
            self.position.z += movement.z;

            self.look_at.x += movement.x;
            self.look_at.y += movement.y;
            self.look_at.z += movement.z;

            // Update the actual scene camera
            let mut scene_lock = scene.lock().unwrap();
            scene_lock
                .get_camera_mut()
                .set_position(self.position.clone().into());
            scene_lock
                .get_camera_mut()
                .set_look_at(self.look_at.clone().into());
            changed = true;
        }

        changed
    }
}

impl Viewable for ProxyCamera {
    type RealSceneObject = Arc<Mutex<Scene>>;

    fn ui(&mut self, ui: &mut Ui, scene: &mut Self::RealSceneObject) -> bool {
        let mut changed = false;

        ui.collapsing(
            RichText::new("ℹ Controls & Tips").color(Color32::LIGHT_BLUE),
            |ui| {
                ui.label(RichText::new("Viewfinding:").strong());
                ui.horizontal(|ui| {
                    ui.label("W/S: Forward/Backward");
                    ui.label("A/D: Left/Right");
                });
                ui.horizontal(|ui| {
                    ui.label("E/Q: Up/Down");
                });
                ui.add_space(5.0);
                ui.label(
                    RichText::new("⚠ Performance")
                        .color(Color32::ORANGE)
                        .strong(),
                );
                ui.label(
                    RichText::new("Reduce samples/resolution for smoother navigation.").small(),
                );
            },
        );

        ui.separator();

        // TODO: Get the fov limits from somewhere central as consts.
        if ui
            .add(egui::Slider::new(&mut self.pane_width, 0.1..=100.0).text("Pane Width"))
            .changed()
        {
            // TODO MICHAEL:
            scene
                .lock()
                .unwrap()
                .get_camera_mut()
                .set_pane_width(self.pane_width);
            changed = true;
        }

        if ui
            .add(egui::Slider::new(&mut self.pane_distance, 0.1..=100.0).text("Pane Distance"))
            .changed()
        {
            // TODO MICHAEL:
            scene
                .lock()
                .unwrap()
                .get_camera_mut()
                .set_pane_distance(self.pane_distance);
            changed = true;
        }

        ui.horizontal(|ui| {
            ui.label("Width:");
            if ui
                .add(
                    egui::DragValue::new(&mut self.resolution[0]), //.range(ImageResolution::MIN[0]..=ImageResolution::MAX[0]),
                )
                .changed()
            {
                // TODO MICHAEL:
                scene
                    .lock()
                    .unwrap()
                    .get_camera_mut()
                    .set_resolution(Resolution {
                        width: self.resolution[0],
                        height: self.resolution[1],
                    });
                changed = true;
            }
        });

        ui.horizontal(|ui| {
            ui.label("Height:");
            if ui
                .add(
                    egui::DragValue::new(&mut self.resolution[1]), //.range(ImageResolution::MIN[1]..=ImageResolution::MAX[1]),
                )
                .changed()
            {
                // TODO MICHAEL:
                scene
                    .lock()
                    .unwrap()
                    .get_camera_mut()
                    .set_resolution(Resolution {
                        width: self.resolution[0],
                        height: self.resolution[1],
                    });
                changed = true;
            }
        });
        ui.separator();

        ui.label("Camera Position:");
        if vec3_ui(ui, &mut self.position) {
            // TODO MICHAEL:
            scene
                .lock()
                .unwrap()
                .get_camera_mut()
                .set_position(self.position.clone().into());
            changed = true;
        }

        ui.label("Camera Direction:");
        if vec3_ui(ui, &mut self.look_at) {
            // TODO MICHAEL:
            scene
                .lock()
                .unwrap()
                .get_camera_mut()
                .set_look_at(self.look_at.clone().into());
            changed = true;
        }
        changed
    }
}

impl Viewable for Vec<ProxyMesh> {
    type RealSceneObject = Arc<Mutex<Scene>>;

    fn ui(&mut self, ui: &mut Ui, scene: &mut Self::RealSceneObject) -> bool {
        let mut changed = false;
        let enum_meshes = self.iter_mut();
        if enum_meshes.len() == 0 {
            ui.label("No objects in scene.");
        } else {
            for (i, proxy_mesh) in enum_meshes.enumerate() {
                CollapsingHeader::new(format!("Object {}", i))
                    .default_open(false)
                    .show(ui, |ui| {
                        changed |=
                            proxy_mesh.ui(ui, &mut scene.lock().unwrap().get_meshes_mut()[i]);
                    });

                if ui.small_button("remove").clicked() {
                    scene.lock().unwrap().get_meshes_mut().remove(i);
                    self.remove(i);
                    changed = true;
                    break;
                }
            }
        }
        changed
    }
}

impl Viewable for ProxyMesh {
    type RealSceneObject = Mesh;

    fn ui(&mut self, ui: &mut Ui, mesh: &mut Mesh) -> bool {
        let mut changed = false;
        // TODO MICHAEL: Hier weiß ich nicht, wie dein neues interface genau sein soll. Falls die meshes hinter einer private reference liegen und irgendwie über indizes geändert werden, müssen wir uns hier etwas neues überlegen.

        ui.label("Rotation:");
        if vec3_ui(ui, &mut self.rotation) {
            mesh.rotate(self.rotation.clone().into()); // TODO MICHAEL: this is probably wrong? Check bitte diese Rotations ab. Falls das hier korrekt ist, einfach die todo kommentare entfernen.
            changed = true;
        }

        ui.label("Scale:");
        if vec3_ui(ui, &mut self.scale) {
            mesh.scale(self.scale.x);
            changed = true;
        }

        ui.label("Translation:");
        if vec3_ui(ui, &mut self.translation) {
            mesh.translate(self.translation.clone().into());
            changed = true;
        }
        changed
    }
}

impl Viewable for Vec<ProxySphere> {
    type RealSceneObject = Arc<Mutex<Scene>>;

    fn ui(&mut self, ui: &mut Ui, scene: &mut Self::RealSceneObject) -> bool {
        let mut changed = false;
        for (i, proxy_sphere) in self.iter_mut().enumerate() {
            CollapsingHeader::new(format!("Sphere {}", i))
                .default_open(false)
                .show(ui, |ui| {
                    changed |= proxy_sphere.ui(ui, &mut scene.lock().unwrap().get_spheres_mut()[i]);
                });

            if ui.small_button("remove").clicked() {
                scene.lock().unwrap().get_spheres_mut().remove(i);
                self.remove(i);
                changed = true;
                break;
            }
        }
        if ui.small_button("+").clicked() {
            let new_sphere = ProxySphere::default();
            scene.lock().unwrap().add_sphere(Sphere::new(
                new_sphere.center.clone().into(),
                new_sphere.radius,
                Material::default(),
                new_sphere.color.clone().into(),
            ));
            self.push(new_sphere);
            changed = true;
        }
        changed
    }
}

impl Viewable for ProxySphere {
    type RealSceneObject = Sphere;

    fn ui(&mut self, ui: &mut Ui, sphere: &mut Sphere) -> bool {
        let mut changed = false;
        ui.label("Radius:");
        if ui
            .add(egui::DragValue::new(&mut self.radius).speed(0.1))
            .changed()
        {
            sphere.set_radius(self.radius);
            changed = true;
        }

        ui.label("Center:");
        if vec3_ui(ui, &mut self.center) {
            sphere.set_center(self.center.clone().into());
            changed = true;
        }

        ui.label("Color:");
        if color_ui(ui, &mut self.color) {
            sphere.set_color(self.color.clone().into());
            changed = true;
        }
        changed
    }
}

impl Viewable for Misc {
    type RealSceneObject = Arc<Mutex<Scene>>;

    fn ui(&mut self, ui: &mut Ui, scene: &mut Self::RealSceneObject) -> bool {
        let mut changed = false;
        if ui
            .checkbox(&mut self.color_hash_enabled, "Enable Color Hash")
            .changed()
        {
            // TODO MICHAEL: die Methode wird zwar aufgerufen ändert aber tatsächlich nichts - schau das bitte an, kann aber auch sein, dass das bereits gefixt wurde.
            scene
                .lock()
                .unwrap()
                .set_color_hash_enabled(self.color_hash_enabled);
            changed = true;
        }

        if ui
            .add(egui::Slider::new(&mut self.ray_samples, 1..=2000).text("Samples"))
            .changed()
        {
            scene
                .lock()
                .unwrap()
                .get_camera_mut()
                .set_ray_samples(self.ray_samples);
            changed = true;
        }

        ui.vertical(|ui| {
            ui.label("Spheres");
            changed |= self.spheres.ui(ui, scene);
        });
        changed
    }
}

impl Viewable for Vec<ProxyLight> {
    type RealSceneObject = Arc<Mutex<Scene>>;

    fn ui(&mut self, ui: &mut Ui, scene: &mut Self::RealSceneObject) -> bool {
        let mut changed = false;
        let enum_light = self.iter_mut();
        if enum_light.len() == 0 {
            ui.label("No lights in scene.");
        } else {
            for (i, proxy_light) in enum_light.enumerate() {
                CollapsingHeader::new(format!("Light {}", i))
                    .default_open(false)
                    .show(ui, |ui| {
                        changed |= proxy_light
                            .ui(ui, &mut scene.lock().unwrap().get_light_sources_mut()[i]);
                    });

                if ui.small_button("remove").clicked() {
                    scene.lock().unwrap().get_light_sources_mut().remove(i);
                    self.remove(i);
                    changed = true;
                    break;
                }
            }
        }

        if ui.small_button("+").clicked() {
            let new_light = ProxyLight::default();
            scene
                .lock()
                .unwrap()
                .add_lightsource(new_light.clone().into());
            self.push(new_light);
            changed = true;
        }
        changed
    }
}

impl Viewable for ProxyLight {
    type RealSceneObject = LightSource;

    fn ui(&mut self, ui: &mut Ui, light: &mut LightSource) -> bool {
        let mut changed = false;
        ui.label("Position:");
        if vec3_ui(ui, &mut self.position) {
            light.set_position(self.position.clone().into());
            changed = true;
        }
        ui.label("Color:");
        if color_ui(ui, &mut self.color) {
            light.set_color(self.color.clone().into());
            changed = true;
        }

        if ui
            .add(egui::Slider::new(&mut self.luminosity, 0.1..=500.0).text("Luminosity"))
            .changed()
        {
            light.set_luminosity(self.luminosity);
            changed = true;
        }
        changed
    }
}
