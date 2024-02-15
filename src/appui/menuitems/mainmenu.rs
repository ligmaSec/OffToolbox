use egui::Ui;
use egui::Context;
use log::debug;

pub fn default(ctx: &Context, ui: &mut Ui){
    Ui::label(ui, "Welcome to the main menu!");
    if ctx.input(|i| i.key_pressed(egui::Key::A)) {
        debug!("A key pressed");
}
}
