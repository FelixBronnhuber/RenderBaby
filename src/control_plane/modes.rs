use std::sync::OnceLock;
use clap::{Parser, Subcommand};
use log::{info, warn};
use crate::control_plane::app::App;

mod benchmark;
pub mod cli_static;
pub mod gui;

static DEBUG_MODE: OnceLock<bool> = OnceLock::new();

pub fn is_debug_mode() -> bool {
    *DEBUG_MODE.get().unwrap_or(&false)
}

#[derive(Subcommand, Debug)]
enum Mode {
    Cli,
    Gui,
    Benchmark,
}

#[derive(Parser, Debug)]
#[command(name = "mode", about = "Choose application mode.")]
struct ModeArg {
    #[command(subcommand)]
    pub mode: Option<Mode>,
    #[arg(long = "no-debug", help = "Disable debug mode.")]
    // TODO: change to "debug" default to false in release mode!!!
    pub no_debug: bool,
}

pub fn get_app() -> Box<dyn App> {
    // Default args
    let mode_arg = ModeArg::try_parse().unwrap_or_else(|e| {
        warn!("No mode provided, defaulting to Gui (--gui). This is because unwrap came with this error: {:?}.", e);
        ModeArg { mode: Some(Mode::Gui), no_debug: false } // TODO: change default debug to false in release mode!!!
    });

    // Set debug mode statically
    DEBUG_MODE
        .set(!mode_arg.no_debug)
        .expect("Failed to set debug mode.");
    info!(
        "Debug mode is {}",
        if is_debug_mode() {
            "enabled"
        } else {
            "disabled"
        }
    );

    // Return the appropriate app based on the selected mode
    match mode_arg.mode {
        Some(Mode::Cli) => Box::new(cli_static::CliStaticApp::new()),
        Some(Mode::Gui) => Box::new(gui::GuiApp::new()),
        Some(Mode::Benchmark) => Box::new(benchmark::BenchmarkApp::new()),
        None => Box::new(gui::GuiApp::new()),
    }
}
