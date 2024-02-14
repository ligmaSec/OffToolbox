use egui::Ui;
use egui::Context;

pub fn default(ctx: &Context, ui: &mut Ui){
   egui::widgets::global_dark_light_mode_buttons(ui)

}
