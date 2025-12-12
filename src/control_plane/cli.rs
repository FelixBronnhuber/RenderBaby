use clap::{Parser, Subcommand};
use std::path::PathBuf;
use log::{error, info};
use crate::data_plane::scene::render_scene::Scene;

#[derive(Subcommand, Debug)]
enum Mode {
    Cli(Args),
    Gui,
}

#[derive(Parser, Debug)]
#[command(name = "mode", about = "Choose application mode.")]
struct ModeArg {
    #[command(subcommand)]
    pub mode: Option<Mode>,
}

#[derive(Parser, Debug)]
#[command(
    name = "cli args",
    about = "Specify command line arguments for the application."
)]
pub struct Args {
    #[arg(long, required = true, help = "Path to the scene json file.")]
    pub scene: PathBuf,

    #[arg(long, required = true, help = "Path to which the image file is saved.")]
    pub output: PathBuf,

    #[arg(
        long,
        default_value = "1080",
        help = "Width of the rendered image in pixels."
    )]
    pub width: u32,

    #[arg(
        long,
        default_value = "720",
        help = "Height of the rendered image in pixels."
    )]
    pub height: u32,

    #[arg(long, default_value = "png", value_parser = ["png", "jpg"], help = "Image file format.")]
    pub filetype: String,
}

pub struct CliApp {
    pub args: Args,
}

impl CliApp {
    pub fn parse() -> Option<Self> {
        match ModeArg::try_parse() {
            Ok(mode_arg) => match mode_arg.mode {
                Some(Mode::Cli(cli_args)) => {
                    info!("CLI mode selected with args: {:?}", cli_args);
                    Some(CliApp { args: cli_args })
                }
                Some(Mode::Gui) => {
                    info!("GUI mode selected.");
                    None
                }
                None => {
                    info!("No mode selected. Defaulting to GUI mode.");
                    None
                }
            },
            Err(e) => {
                error!("Found CLI mode but had error parsing arguments, {}", e);
                std::process::exit(1);
            }
        }
    }

    pub fn run(&self) {
        info!("Loading scene...");

        let scene_res = Scene::load_scene_from_file(self.args.scene.clone());
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
