use egui::Ui;
use egui::Context;
use lazy_static::lazy_static;
use std::sync::Mutex;

// state 1: ARP
// state 2: 
// state 3: 
// state 4: 
//
//
#[derive(serde::Deserialize, serde::Serialize)]
pub enum NetworkMenuStates {
    ARP,
    State2,
    State3,
    State4,
}


#[derive(serde::Deserialize, serde::Serialize)]
pub struct NetworkMenuState {
    current_state: NetworkMenuStates,
}

// Use lazy_static to initialize a mutable global variable
lazy_static! {
    static ref NETWORK_MENU_STATE: Mutex<NetworkMenuState> = Mutex::new(NetworkMenuState {
        current_state: NetworkMenuStates::ARP,
    });
}
static mut RADIO_VALUE: bool = false;
pub fn default(ctx: &Context, ui: &mut Ui){

    ui.heading("Network");

    ui.horizontal_top(|ui|{
        // Use lock() to access the mutable global variable
        let mut network_menu_state = NETWORK_MENU_STATE.lock().unwrap();
        if ui.button("ARP").on_hover_text("Address Resolution Protocol").clicked() {
            network_menu_state.current_state = NetworkMenuStates::ARP;

        }
        //TODO: add more menus
        if ui.button("2").on_hover_text("Address Resolution Protocol").clicked() {
            network_menu_state.current_state = NetworkMenuStates::State2;
        }
        if ui.button("3").on_hover_text("Address Resolution Protocol").clicked() {
            network_menu_state.current_state = NetworkMenuStates::State3;
        }
        if ui.button("4").on_hover_text("Address Resolution Protocol").clicked() {
            network_menu_state.current_state = NetworkMenuStates::State4;
        }

    });


    ui.separator();




    match NETWORK_MENU_STATE.lock().unwrap().current_state {
        NetworkMenuStates::ARP => arp(ctx, ui),
        NetworkMenuStates::State2 => state2(ctx, ui),
        NetworkMenuStates::State3 => state3(ctx, ui),
        NetworkMenuStates::State4 => state4(ctx, ui),
    }
}




fn arp(ctx: &Context, ui: &mut Ui){
    ui.horizontal( |ui| {
        ui.label("ARP Mode");
        //ui.radio_value(&mut RADIO_VALUE, false, "Active");
        //ui.radio_value(&mut RADIO_VALUE, true, "Passive");
    });
}


fn state2(ctx: &Context, ui: &mut Ui){
    ui.label("this is the menu 2");
}

fn state3(ctx: &Context, ui: &mut Ui){
    ui.label("this is the menu 3 ");
}

fn state4(ctx: &Context, ui: &mut Ui){
    ui.label("this is the menu 4");
}

