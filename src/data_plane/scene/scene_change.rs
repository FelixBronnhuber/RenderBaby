use glam::Vec3;
use scene_objects::camera::Resolution;

pub(crate) enum SceneChange {
    CameraChange(CameraChange),
    LightChange,
    VerticesChange,
    TriangleChange,
    SphereChange,
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
