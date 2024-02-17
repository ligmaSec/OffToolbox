use egui::Ui;
use egui::Context;

// state 1: ARP
// state 2: 
// state 3: 
// state 4: 
//
//
#[derive(serde::Deserialize, serde::Serialize, Default)]
enum NetworkMenuStates {
    #[default]
    ARP,
    State2,
    State3,
    State4,
}


#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct NetworkMenuState {
    current_state: NetworkMenuStates,
}

static mut NETWORK_MENU_STATE: NetworkMenuState = NetworkMenuState {
    current_state: NetworkMenuStates::ARP,
};
pub fn default(ctx: &Context, ui: &mut Ui){

    ui.heading("Network");

    unsafe {
    ui.horizontal_top(|ui|{
        if ui.button("ARP").on_hover_text("Address Resolution Protocol").clicked() {
            NETWORK_MENU_STATE.current_state = NetworkMenuStates::ARP;

        }
        
        //horizontal button
        if ui.button("2").on_hover_text("Address Resolution Protocol").clicked() {
            NETWORK_MENU_STATE.current_state = NetworkMenuStates::State2;
        }
        if ui.button("3").on_hover_text("Address Resolution Protocol").clicked() {
            NETWORK_MENU_STATE.current_state = NetworkMenuStates::State3;
        }
        if ui.button("4").on_hover_text("Address Resolution Protocol").clicked() {
            NETWORK_MENU_STATE.current_state = NetworkMenuStates::State4;
        }

    });


    ui.separator();




    match NETWORK_MENU_STATE.current_state {
        NetworkMenuStates::ARP => arp(ctx, ui),
        NetworkMenuStates::State2 => state2(ctx, ui),
        NetworkMenuStates::State3 => state3(ctx, ui),
        NetworkMenuStates::State4 => state4(ctx, ui),
    }
    }
    }




fn arp(ctx: &Context, ui: &mut Ui){
    ui.label("this is the arp menu");
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
