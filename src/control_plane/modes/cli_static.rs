use clap::Parser;
use std::path::PathBuf;
use log::{error, info};
use crate::control_plane::app::App;
use crate::data_plane::scene::render_scene::Scene;
use crate::included_files::AutoPath;

#[derive(Parser, Debug)]
#[command(
    name = "cli args",
    about = "Specify command line arguments for the application."
)]
struct Args {
    #[arg(long, required = true, help = "Path to the scene json file.")]
    pub scene: PathBuf,

    #[arg(long, required = true, help = "Path to which the image file is saved.")]
    pub output: PathBuf,

    #[arg(long, default_value = "png", value_parser = ["png", "jpg"], help = "Image file format.")]
    pub filetype: String,
}

pub struct CliStaticApp {
    args: Args,
}

impl App for CliStaticApp {
    fn new() -> Self {
        let args = match Args::try_parse() {
            Ok(args) => args,
            Err(e) => {
                error!("Error parsing command line arguments: {:?}, exiting...", e);
                std::process::exit(1);
            }
        };
        Self { args }
    }

    fn show(self: Box<CliStaticApp>) {
        info!("Loading scene...");

        let auto_path = match AutoPath::try_from(self.args.scene.clone()) {
            Ok(path) => path,
            Err(e) => {
                error!("Error loading scene: {:?}, exiting...", e);
                std::process::exit(1);
            }
        };

        let scene_res = Scene::load_scene_from_path(auto_path.clone(), true);
        let mut scene: Scene;
        match scene_res {
            Err(e) => {
                error!("Error loading scene: {:?}, exiting...", e);
                std::process::exit(1);
            }
            Ok(s) => {
                scene = s;
                info!("Finished loading scene, starting render...");
            }
        }

        match scene.render() {
            Err(e) => {
                error!("Error rendering scene: {:?}, exiting...", e);
                std::process::exit(1);
            }
            Ok(_) => {
                info!("Finished rendering scene, saving image");
            }
        }

        match scene.export_render_img(self.args.output.clone()) {
            Err(e) => {
                error!("Error saving image: {:?}, exiting...", e);
                std::process::exit(1);
            }
            Ok(_) => {
                info!("Finished saving image.");
            }
        }

        info!("Saved image to {:?}. Exiting...", self.args.output);
    }
}
