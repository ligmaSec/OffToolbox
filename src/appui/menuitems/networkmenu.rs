use egui::Ui;
use egui::Context;
use crate::core;


// state 1: ARP
// state 2: 
// state 3: 
// state 4: 
//

#[derive(serde::Deserialize, serde::Serialize)]
pub struct NetworkMenu {
    module_state: NetworkMenuStates,
    arp: core::arp::ArpModes,
}


#[derive(serde::Deserialize, serde::Serialize)]
pub enum NetworkMenuStates {
    ARP,
    State2,
    State3,
    State4,
}

impl Default for NetworkMenu {
    fn default() -> Self {
        Self {
            module_state: NetworkMenuStates::ARP,
            arp: core::arp::ArpModes::Passive,
        }
    }
}

impl super::View for NetworkMenu {
    fn ui(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.heading("Network");

        ui.horizontal_top(|ui|{
            if ui.button("ARP").on_hover_text("Address Resolution Protocol").clicked() {
                self.module_state = NetworkMenuStates::ARP;

            }
            //TODO: add more menus
            if ui.button("2").on_hover_text("Address Resolution Protocol").clicked() {
                self.module_state = NetworkMenuStates::State2;
            }
            if ui.button("3").on_hover_text("Address Resolution Protocol").clicked() {
                self.module_state = NetworkMenuStates::State3;
            }
            if ui.button("4").on_hover_text("Address Resolution Protocol").clicked() {
                self.module_state = NetworkMenuStates::State4;
            }

        });


        ui.separator();




        match self.module_state {
            NetworkMenuStates::ARP => arp(ctx, ui),
            NetworkMenuStates::State2 => state2(ctx, ui),
            NetworkMenuStates::State3 => state3(ctx, ui),
            NetworkMenuStates::State4 => state4(ctx, ui),
        }

        }
}



fn arp(ctx: &Context, ui: &mut Ui){


    ui.horizontal( |ui| {
        ui.label("ARP Mode");
        ui.radio_value(&mut core::arp::ArpModes::default(), core::arp::ArpModes::Active, "Active");
        ui.radio_value(&mut core::arp::ArpModes::default(), core::arp::ArpModes::Passive, "Passive");
        //why is this not working?
    });
    ui.end_row();
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

