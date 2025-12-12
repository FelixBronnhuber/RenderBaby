use egui::{Area, Color32, Context, Id, LayerId, Order, Sense};

pub fn color_overlay(ctx: &Context, rgba: Color32, click_through: bool) {
    let rect = ctx.content_rect();
    let layer_id = LayerId::new(Order::Middle, Id::new("overlay"));
    Area::new(layer_id.id)
        .order(Order::Middle)
        .fixed_pos(rect.min)
        .show(ctx, |ui| {
            if click_through {
                ui.allocate_rect(rect, Sense::click_and_drag());
            }
            ui.painter().rect_filled(rect, 0.0, rgba);
        });
}
