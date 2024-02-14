use egui::Ui;
use egui::Context;


pub fn default(ctx: &Context, ui: &mut Ui){
    Ui::label(ui, "Welcome to the main menu!");
    if ctx.input(|i| i.key_pressed(egui::Key::A)) {
        println!("A key pressed!");
}
}
