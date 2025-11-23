// This is for the scene file parser
mod scene_parser;

use crate::scene_parser::parse_scene;
use scene::scene::Scene;
use scene::{Scene_Parse_Trait, call_scene_parse};
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
    #[test]
    fn parse_json() {}
}
pub struct Scene_Call;
impl Scene_Parse_Trait for Scene_Call {
    fn do_scene_parse(&self, path: String) -> Scene {
        parse_scene(path)
    }
}
pub fn create_scene_parser() -> Scene_Call {
    Scene_Call
}
