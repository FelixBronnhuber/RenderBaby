use crate::ViewWrapper;
use eframe::egui::Context;
use eframe::{App, CreationContext, Frame};

pub trait EframeViewWrapper<E, P>: ViewWrapper<E, P> + App + 'static {
    fn on_start(&mut self, ctx: &Context, frame: &mut Frame);

    fn open_native(self, app_name: &str) {
        let options = eframe::NativeOptions::default();
        let _ = eframe::run_native(
            app_name,
            options,
            Box::new(move |_cc: &CreationContext| -> Result<Box<dyn App>, Box<dyn std::error::Error + Send + Sync>> {
                struct Wrapper<T: App + EframeViewWrapper<E, P>, E, P> {
                    inner: T,
                    started: bool,
                    _phantom: std::marker::PhantomData<(E, P)>,
                }

                impl<T, E, P> App for Wrapper<T, E, P>
                where
                    T: App + EframeViewWrapper<E, P>,
                {
                    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
                        if !self.started {
                            self.inner.on_start(ctx, frame);
                            self.started = true;
                        }
                        self.inner.update(ctx, frame);
                    }
                }

                Ok(Box::new(Wrapper::<Self, E, P> {
                    inner: self,
                    started: false,
                    _phantom: std::marker::PhantomData,
                }) as Box<dyn App>)
            }),
        );
    }
}
