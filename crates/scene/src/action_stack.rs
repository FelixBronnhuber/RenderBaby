pub struct ActionStack {
    actions: Vec<SceneAction>,
    undone: Vec<SceneAction>
}

impl ActionStack {
    pub fn undo(&mut self) {todo!()}
    pub fn redo(&mut self) {todo!()}
}


pub struct SceneAction {

}