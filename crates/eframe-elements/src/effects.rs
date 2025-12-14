use egui::{vec2, Area, Color32, Context, Id, LayerId, Order, Sense, Ui};

pub trait Effect {
    fn reset(&mut self) {}
    fn ui(&mut self, ui: &mut Ui);
    fn update(&mut self, ctx: &Context);
}

type FillEffectAdditionalFn = Box<dyn FnMut(&mut Ui, egui::Rect)>;

pub struct FillEffect {
    pub id: LayerId,
    pub rgba: Color32,
    pub click_through: bool,
    pub additional_fn: Option<FillEffectAdditionalFn>,
}

impl FillEffect {
    pub(crate) fn new(
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

    fn _update(&mut self, ctx: &Context, rect: egui::Rect) {
        Area::new(self.id.id)
            .order(Order::Middle)
            .fixed_pos(rect.min)
            .show(ctx, |ui| {
                ui.set_min_size(rect.size());

                if !self.click_through {
                    ui.allocate_rect(rect, Sense::click_and_drag());
                }

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

pub struct LoadingEffect {
    fill_effect: FillEffect,
}

impl LoadingEffect {
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
