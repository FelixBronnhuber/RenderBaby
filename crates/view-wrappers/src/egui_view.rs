use crate::ViewWrapper;
use eframe::egui::Context;
use eframe::{App, CreationContext, Frame};

/// Trait for wrappers around views that can be opened with eframe
pub trait EframeViewWrapper: ViewWrapper + App + 'static {
    /// Called in the first [`App::update`] cycle.
    fn on_start(&mut self, ctx: &Context, frame: &mut Frame);

    /// Opens the view using eframe (native settings, Wgpu renderer for stability).
    fn open_native(self, app_name: &str) {
        let options = eframe::NativeOptions {
            renderer: eframe::Renderer::Wgpu,
            ..Default::default()
        };
        let _ = eframe::run_native(
            app_name,
            options,
            Box::new(move |_cc: &CreationContext| -> Result<Box<dyn App>, Box<dyn std::error::Error + Send + Sync>> {

                struct Wrapper<T: EframeViewWrapper> {
                    inner: T,
                    started: bool,
                }

                impl<T> App for Wrapper<T>
                where
                    T: EframeViewWrapper,
                {
                    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
                        if !self.started {
                            self.inner.on_start(ctx, frame);
                            self.started = true;
                        }
                        self.inner.update(ctx, frame);
                    }
                }

                Ok(Box::new(Wrapper::<Self> {
                    inner: self,
                    started: false,
                }) as Box<dyn App>)
            }),
        );
    }
}
