// stores all actions, can be called to undo them and will store undone actions, so they can be appleid again
pub struct  ActionStack{
    done: u8, // todo: use scene_action
    undone: u8
}
impl ActionStack{
    pub fn push(&self, action: u8) {}
    pub fn undo(&self) {}
}