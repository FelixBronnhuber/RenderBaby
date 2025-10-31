//! The scene plane holds all scene objects...
//! 
mod scene_geometry;
mod scene_graph;
mod action_stack;
mod scene_action;
mod scene;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}


