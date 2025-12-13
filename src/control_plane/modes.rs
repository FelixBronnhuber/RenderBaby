use clap::{Parser, Subcommand};
use log::warn;
use crate::control_plane::app::App;

pub mod cli_static;
pub mod gui;

#[derive(Subcommand, Debug)]
enum Mode {
    Cli,
    Gui,
}

#[derive(Parser, Debug)]
#[command(name = "mode", about = "Choose application mode.")]
struct ModeArg {
    #[command(subcommand)]
    pub mode: Option<Mode>,
}

pub fn get_app() -> Box<dyn App> {
    let mode_arg = ModeArg::try_parse().unwrap_or_else(|e| {
        warn!("No mode provided, defaulting to Gui (--gui). This is because unwrap came with this error: {:?}.", e);
        ModeArg { mode: Some(Mode::Gui) }
    });
    match mode_arg.mode {
        Some(Mode::Cli) => Box::new(cli_static::CliStaticApp::new()),
        Some(Mode::Gui) => Box::new(gui::GuiApp::new()),
        None => Box::new(gui::GuiApp::new()),
    }
}
