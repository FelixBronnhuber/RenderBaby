#![allow(dead_code)]
mod action_stack;
pub mod geometric_object;
pub mod obj_parser;
pub mod scene;
pub mod scene_engine_adapter;
mod scene_graph;
mod scene_parser;

#[cfg(test)]
mod tests {
    use crate::scene::Scene;

    #[test]
    fn test_proto_init() {
        // TODO: Instantiating a Scene also instantiates an Engine, which fails on the CI server,
        // since it does not have a GPU.
        // let mut scene = Scene::new();
        // scene.proto_init();
        assert_eq!(4, 4)
    }
}
