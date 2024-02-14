use egui::Ui;
use egui::Window;
use egui::Context;

pub fn default(ctx: &Context, ui: &mut Ui) {
    Ui::vertical_centered(ui, |ui| {
        Ui::label(ui, "Are you sure you want to quit?");
        if ui.button("Yes").clicked() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        });
    }
    


