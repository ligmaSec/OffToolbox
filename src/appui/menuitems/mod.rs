pub mod mainmenu;
pub mod settingsmenu;
pub mod aboutmenu;
pub mod quitmenu;
pub mod networkmenu;


pub trait View {
    fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui);
}
