use crate::traits::View;
use eframe::egui::Context;
use eframe::{App, CreationContext, Frame};

pub trait EframeView: View + App + 'static {
    fn on_start(&mut self, ctx: &Context, frame: &mut Frame);

    fn open_native(self, app_name: &str) {
        let options = eframe::NativeOptions::default();
        let _ = eframe::run_native(
            app_name,
            options,
            Box::new(move |_cc: &CreationContext| -> Result<Box<dyn App>, Box<dyn std::error::Error + Send + Sync>> {
                struct Wrapper<T: EframeView> {
                    inner: T,
                    started: bool,
                }

                impl<T: EframeView> App for Wrapper<T> {
                    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
                        if !self.started {
                            self.inner.on_start(ctx, frame);
                            self.started = true;
                        }
                        self.inner.update(ctx, frame);
                    }
                }

                Ok(Box::new(Wrapper {
                    inner: self,
                    started: false,
                }) as Box<dyn App>)
            }),
        );
    }
}
