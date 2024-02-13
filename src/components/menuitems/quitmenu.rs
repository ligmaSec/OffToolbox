use egui::Ui;
use egui::Window;


pub fn default(ui: &mut Ui) {

    Window::new("Quit Menu")
        .collapsible(false)
        .resizable(false)
        .hscroll(false)
        .show(ui.ctx(), |ui| {
            ui.label("Are you sure you want to quit?");
            if ui.button("Yes").clicked() {
                std::process::exit(0);
            }
            if ui.button("No").clicked() {
            }
        });
}

