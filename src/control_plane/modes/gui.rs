use engine_wgpu_wrapper::GpuDevice;
use view_wrappers::ViewWrapper;
use crate::control_plane::app::App;
use crate::control_plane::modes::is_debug_mode;

pub mod model;
mod screens;
pub mod view;

pub struct GuiApp {
    view: view::View,
}

impl GuiApp {
    pub fn new() -> Self {
        // fixes device creation bug on Windows AMD:
        let _ = GpuDevice::new();

        let view = view::View::new();

        GuiApp { view }
    }
}

impl App for GuiApp {
    fn show(self: Box<GuiApp>) {
        self.view.open();
    }
}
