use crate::data_plane::{scene::render_scene::RealScene, scene_proxy::proxy_scene::ProxyScene};

pub(super) struct Scene {
    real_scene: RealScene,
    proxy_scene: ProxyScene,
}
