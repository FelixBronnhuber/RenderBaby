use crate::{scene, scene_geometry, scene_graph::scene_graph};
use log::{info};

// Scene actions are actions that are applied to the scene or the elements of the scene
// They need to be added to the ActionStack when applied
// they should offer a loggable debug string?
pub trait SceneAction{
    fn description(&self) -> &str;
    fn action(&self);
    fn target(&self);
}

pub struct AddObjectToSzene<'a>{
    text: &'a str,
    target: &'a scene_graph::SceneGraph<'a>,
    object: &'a dyn scene_geometry::SceneGeometry
}

impl <'a>SceneAction for AddObjectToSzene<'a>{
    fn description(&self) -> &str {
        todo!()
    }

    fn action(&self) {
        self.target.add(self.object);
    }

    fn target(&self) {
        todo!()
    }
}
pub struct ActionHandler<'a>{
    szene: &'a scene::scene::Scene<'a>
} // for logging, pushing ...
impl <'a> ActionHandler<'a> {
    pub fn handle(&self, action: &dyn SceneAction){
        info!("{}", action.description().to_owned());

        action.action(); // probably Result...

        // todo: push to actionstack!

        info!("Success? ")
        //Ok()

    }
}