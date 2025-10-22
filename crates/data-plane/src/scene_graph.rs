pub mod scene_graph {
    use crate::scene_geometry::SceneGeometry;

    pub struct SceneGraph<'a> {
        // elements pub as cheap solution for now...
        pub elements: &'a mut Vec<dyn SceneGeometry>
    }

}