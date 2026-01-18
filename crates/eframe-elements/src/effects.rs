use egui::{vec2, Area, Color32, Context, Id, LayerId, Order, Sense, Ui};

/// Effect trait that should be drawn on top of any ui element.
pub trait Effect {
    /// Reset effect to its initial state.
    fn reset(&mut self) {}

    /// Apply the effect to an [`Ui`] element.
    fn ui(&mut self, ui: &mut Ui);

    /// Apply the effect to the entire window.
    fn update(&mut self, ctx: &Context);
}

/// Function that is called to draw additional content on top of a [`FillEffect`].
///
/// Receives the [`Ui`] and the filled [`egui::Rect`].
type FillEffectAdditionalFn = Box<dyn FnMut(&mut Ui, egui::Rect)>;

/// Fills the UI with a solid rgba color.
pub struct FillEffect {
    /// The layer id of the fill effect.
    pub id: LayerId,
    /// Color used to fill the UI.
    pub rgba: Color32,
    /// Whether the fill effect should be click-through.
    pub click_through: bool,
    /// Optional function that can be called to draw additional content on top of the fill effect.
    pub additional_fn: Option<FillEffectAdditionalFn>,
}

impl FillEffect {
    /// Create a new [`FillEffect`].
    pub fn new(
        id: Id,
        rgba: Color32,
        click_through: bool,
        additional_fn: Option<FillEffectAdditionalFn>,
    ) -> Self {
        let id = LayerId::new(Order::Foreground, id);
        Self {
            id,
            rgba,
            click_through,
            additional_fn,
        }
    }

    /// Internal helper function that draws the fill effect to the given [`egui::Rect`].
    fn _update(&mut self, ctx: &Context, rect: egui::Rect) {
        Area::new(self.id.id)
            .order(Order::Middle)
            .fixed_pos(rect.min)
            .interactable(self.click_through)
            .show(ctx, |ui| {
                ui.set_min_size(rect.size());
                ui.painter().rect_filled(rect, 0.0, self.rgba);
                if let Some(extra) = self.additional_fn.as_mut() {
                    extra(ui, rect);
                }
            });
    }
}

impl Effect for FillEffect {
    fn ui(&mut self, ui: &mut Ui) {
        let ctx = ui.ctx();
        let rect = ui.clip_rect();
        self._update(ctx, rect);
    }

    fn update(&mut self, ctx: &Context) {
        let rect = ctx.content_rect();
        self._update(ctx, rect);
    }
}

/// Fill effect extension that draws a spinner/loading animation.
pub struct LoadingEffect {
    /// Owns a [`FillEffect`] that draws the spinner.
    fill_effect: FillEffect,
}

impl LoadingEffect {
    /// Create a new [`LoadingEffect`].
    ///
    /// Receives an effect [`Id`] and the spinner size in pixels ([`f32`]).
    pub fn new(id: Id, spinner_size: f32) -> Self {
        let mut temp_self = Self {
            fill_effect: FillEffect::new(
                id,
                Color32::from_rgba_unmultiplied(0, 0, 0, 60),
                false,
                None,
            ),
        };

        temp_self.fill_effect.additional_fn =
            Some(Box::new(move |ui: &mut Ui, rect: egui::Rect| {
                let center = rect.center();
                let spinner_rect =
                    egui::Rect::from_center_size(center, vec2(spinner_size, spinner_size));

                ui.allocate_rect(spinner_rect, Sense::hover());

                ui.scope(|ui| {
                    ui.set_min_size(spinner_rect.size());
                    ui.add(egui::Spinner::new().size(spinner_size))
                });
            }));
        temp_self
    }
}

impl Effect for LoadingEffect {
    fn ui(&mut self, ui: &mut Ui) {
        self.fill_effect.ui(ui);
    }

    fn update(&mut self, ctx: &Context) {
        self.fill_effect.update(ctx);
    }
}
