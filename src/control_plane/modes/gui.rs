use engine_wgpu_wrapper::GpuDevice;
use view_wrappers::ViewWrapper;
use crate::control_plane::app::App;

pub mod model;
mod screens;
pub mod view;

pub struct GuiApp {
    view: view::View,
}

impl App for GuiApp {
    fn new() -> Self {
        // fix device creation bug on windows AMD:
        let _ = GpuDevice::new();

        let view = view::View::new();

        GuiApp { view }
    }

    fn show(self: Box<GuiApp>) {
        self.view.open();
    }
}
