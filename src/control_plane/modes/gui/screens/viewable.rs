use egui::{CollapsingHeader, Ui};
use scene_objects::camera::Resolution;
use scene_objects::geometric_object::GeometricObject;
use scene_objects::light_source::{LightSource, LightType};
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
    fn ui(&mut self, ui: &mut Ui, object: &mut Self::RealSceneObject);
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
                    .speed(1)
                    .range(0.0..=255.0),
            )
            .changed();
        ui.label("G:");
        changed |= ui
            .add(
                egui::DragValue::new(&mut color.g)
                    .speed(1)
                    .range(0.0..=255.0),
            )
            .changed();
        ui.label("B:");
        changed |= ui
            .add(
                egui::DragValue::new(&mut color.b)
                    .speed(1)
                    .range(0.0..=255.0),
            )
            .changed();
    });
    changed
}

impl Viewable for ProxyCamera {
    type RealSceneObject = Scene;

    fn ui(&mut self, ui: &mut Ui, scene: &mut Scene) {
        ui.label("Camera");

        // TODO: Get the fov limits from somewhere central as consts.
        if ui
            .add(egui::Slider::new(&mut self.pane_width, 0.1..=100.0).text("Pane Width"))
            .changed()
        {
            // TODO MICHAEL:
            scene.get_camera_mut().set_pane_width(self.pane_width);
        }

        if ui
            .add(egui::Slider::new(&mut self.pane_distance, 0.1..=100.0).text("Pane Distance"))
            .changed()
        {
            // TODO MICHAEL:
            scene.get_camera_mut().set_pane_distance(self.pane_distance);
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
                scene.get_camera_mut().set_resolution(Resolution {
                    width: self.resolution[0],
                    height: self.resolution[1],
                });
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
                scene.get_camera_mut().set_resolution(Resolution {
                    width: self.resolution[0],
                    height: self.resolution[1],
                })
            }
        });
        ui.separator();

        ui.label("Camera Position:");
        if vec3_ui(ui, &mut self.position) {
            // TODO MICHAEL:
            scene
                .get_camera_mut()
                .set_position(self.position.clone().into());
        }

        ui.label("Camera Direction:");
        if vec3_ui(ui, &mut self.look_at) {
            // TODO MICHAEL:
            scene
                .get_camera_mut()
                .set_look_at(self.look_at.clone().into());
        }
    }
}

impl Viewable for ProxyMesh {
    type RealSceneObject = Mesh;

    fn ui(&mut self, ui: &mut Ui, mesh: &mut Mesh) {
        // TODO MICHAEL: Hier weiß ich nicht, wie dein neues interface genau sein soll. Falls die meshes hinter einer private reference liegen und irgendwie über indizes geändert werden, müssen wir uns hier etwas neues überlegen.

        ui.label("Rotation:");
        if vec3_ui(ui, &mut self.rotation) {
            mesh.rotate(self.rotation.clone().into()); // TODO MICHAEL: this is probably wrong? Check bitte diese Rotations ab. Falls das hier korrekt ist, einfach die todo kommentare entfernen.
        }

        ui.label("Scale:");
        if vec3_ui(ui, &mut self.scale) {
            mesh.scale(self.scale.x);
        }

        ui.label("Translation:");
        if vec3_ui(ui, &mut self.translation) {
            mesh.translate(self.translation.clone().into());
        }
    }
}

impl Viewable for ProxySphere {
    type RealSceneObject = Sphere;

    fn ui(&mut self, ui: &mut Ui, sphere: &mut Sphere) {
        ui.label("Radius:");
        if ui
            .add(egui::DragValue::new(&mut self.radius).speed(0.1))
            .changed()
        {
            sphere.set_radius(self.radius);
        }

        ui.label("Center:");
        if vec3_ui(ui, &mut self.center) {
            sphere.set_center(self.center.clone().into());
        }

        ui.label("Color:");
        if color_ui(ui, &mut self.color) {
            sphere.set_color(self.color.clone().into());
        }
    }
}

impl Viewable for Misc {
    type RealSceneObject = Scene;

    fn ui(&mut self, ui: &mut Ui, scene: &mut Scene) {
        if ui
            .checkbox(&mut self.color_hash_enabled, "Enable Color Hash")
            .changed()
        {
            // TODO MICHAEL: die Methode wird zwar aufgerufen ändert aber tatsächlich nichts - schau das bitte an, kann aber auch sein, dass das bereits gefixt wurde.
            scene.set_color_hash_enabled(self.color_hash_enabled);
        }

        if ui
            .add(egui::Slider::new(&mut self.ray_samples, 1..=2000).text("Samples"))
            .changed()
        {
            scene.get_camera_mut().set_ray_samples(self.ray_samples);
        }

        ui.vertical(|ui| {
            let real_spheres = scene.get_spheres_mut();
            ui.label("Spheres");
            for (i, proxy_sphere) in self.spheres.iter_mut().enumerate() {
                CollapsingHeader::new(format!("Sphere {}", i))
                    .default_open(false)
                    .show(ui, |ui| {
                        proxy_sphere.ui(ui, &mut real_spheres[i]);
                    });

                if ui.small_button("remove").clicked() {
                    real_spheres.remove(i);
                    self.spheres.remove(i);
                    break;
                }
            }
            if ui.small_button("+").clicked() {
                let new_sphere = ProxySphere::default();
                scene.add_sphere(Sphere::new(
                    new_sphere.center.clone().into(),
                    new_sphere.radius,
                    Material::default(),
                    new_sphere.color.clone().into(),
                ));
                self.spheres.push(new_sphere);
            }
        });
    }
}

impl Viewable for ProxyLight {
    type RealSceneObject = LightSource;

    fn ui(&mut self, ui: &mut Ui, light: &mut LightSource) {
        ui.label("Position:");
        if vec3_ui(ui, &mut self.position) {
            light.set_position(self.position.clone().into());
        }
        ui.label("Rotation:");
        if vec3_ui(ui, &mut self.rotation) {
            light.rotate(self.rotation.clone().into()); // TODO: this is probably wrong!
        }
        ui.label("Color:");
        if color_ui(ui, &mut self.color) {
            light.set_color(self.color.clone().into());
        }

        if ui
            .add(egui::Slider::new(&mut self.luminosity, 0.1..=100.0).text("Luminosity"))
            .changed()
        {
            light.set_luminosity(self.luminosity);
        }

        let light_types: [String; 3] = [
            LightType::Ambient.into(),
            LightType::Directional.into(),
            LightType::Point.into(),
        ];

        egui::ComboBox::from_label("Type")
            .selected_text(&self.light_type)
            .show_ui(ui, |ui| {
                for m in &light_types {
                    if ui
                        .selectable_value(&mut self.light_type, m.clone(), format!("{:?}", m))
                        .changed()
                    {
                        light.set_light_type(self.light_type.clone().into());
                    }
                }
            });
    }
}
