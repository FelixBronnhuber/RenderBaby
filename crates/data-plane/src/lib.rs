//! The scene plane holds all scene objects...
//! 
mod scene_geometry;
mod scene_graph;
mod action_stack;

pub mod scene {
    use crate::{action_stack::ActionStack, scene_graph};

    pub struct Scene<'a> {
        // todo: new, Singleton, maybe with crate...
        pub scene_graph: scene_graph::scene_graph::SceneGraph<'a>,
        action_stack: ActionStack
    }
    impl <'a> Scene <'a> {
        pub fn render() -> u8 {1} // todo type, call something...
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}


