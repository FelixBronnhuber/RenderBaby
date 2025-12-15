use std::path::PathBuf;
use include_dir::*;
/*
static SCENE_TEMPLATES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/scene");
static OBJ_TEMPLATES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/obj");

pub fn list_scene_templates() -> Vec<PathBuf> {
    let res: Vec<PathBuf> = vec![];
    #[cfg(feature = "glob")]
    {
        for entry in crate::included_dirs::SCENE_TEMPLATES.find("*.json").unwrap() {
            res.push(entry.path());
        }
    }
    res
}

pub fn list_obj_template() -> Vec<PathBuf> {
    let res: Vec<PathBuf> = vec![];
    #[cfg(feature = "glob")]
    {
        for entry in crate::included_dirs::SCENE_TEMPLATES.find("*.obj").unwrap() {
            res.push(entry.path());
        }
    }
    res
}
*/
