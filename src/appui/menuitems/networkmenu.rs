use egui::Ui;
use egui::Context;

// state 1: ARP
// state 2: 
// state 3: 
// state 4: 
#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct NetworkMenuState {
    states: [u8; 4],
}
pub fn default(ctx: &Context, ui: &mut Ui){
    ui.button("ARP").on_hover_text("Address Resolution Protocol");

}
