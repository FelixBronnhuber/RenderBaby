use glam::Vec3;
use scene_objects::{camera::Resolution, light_source::LightType, material::Material};

pub(crate) enum SceneChange {
    CameraChange(CameraChange),
    LightChange(LightChange),
    MeshChange(MeshChange),
    SphereChange(SphereChange),
    General,
}

pub(crate) enum CameraChange {
    Position(Vec3),
    LookAt(Vec3),
    Up(Vec3),
    PaneDistance(f32),
    PaneWidth(f32),
    Resolution(Resolution),
    RaySamples(u32),
}

pub(crate) enum LightChange {
    Type(LightType, usize), // maybe not needed
    Position(Vec3, usize),
    Luminosity(f32, usize),
    Color([f32; 3], usize),
    Direction(Vec3, usize),
    Name(String, usize),
}

pub(crate) enum MeshChange {
    Translate(Vec3, usize),
    Scale(f32, usize),
    Rotate(Vec3, usize),
    Material(Material, usize),
    Name(String, usize),
}

pub(crate) enum SphereChange {
    Count,
    Translate(Vec3, usize),
    Scale(f32, usize),
    Color([f32; 3], usize),
    Material(Material, usize),
    Name(String, usize),
}
