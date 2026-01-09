use include_dir::{include_dir, Dir, File};

/* Functions to include files from the binary in the executable - instead of relying on the path.
This makes the executable more reliable. Disagree?
 */

static SCENE_TEMPLATES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/scene");

fn get_all_files_with_extension<'a>(dir: &'a Dir<'a>, extension: &str) -> Vec<&'a File<'a>> {
    dir.files()
        .filter(|f| f.path().extension() == Some(extension.as_ref()))
        .collect()
}

pub fn list_scene_templates() -> Vec<&'static File<'static>> {
    get_all_files_with_extension(&SCENE_TEMPLATES, "json")
}
