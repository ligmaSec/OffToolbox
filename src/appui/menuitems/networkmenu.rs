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
    arp_mode: core::network::arp::ArpModes,
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
            arp_mode: core::network::arp::ArpModes::Passive,
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
            if ui.button("Menu 2").on_hover_text("Address Resolution Protocol").clicked() {
                self.module_state = NetworkMenuStates::State2;
            }
            if ui.button("Menu 3").on_hover_text("Address Resolution Protocol").clicked() {
                self.module_state = NetworkMenuStates::State3;
            }
            if ui.button("Menu 4").on_hover_text("Address Resolution Protocol").clicked() {
                self.module_state = NetworkMenuStates::State4;
            }

        });


        ui.separator();




        match self.module_state {
            NetworkMenuStates::ARP => self.arp(ctx, ui),
            NetworkMenuStates::State2 => self.state2(ctx, ui),
            NetworkMenuStates::State3 => self.state3(ctx, ui),
            NetworkMenuStates::State4 => self.state4(ctx, ui),
        }

        }




}

impl NetworkMenu {


    fn arp(&mut self,ctx: &Context, ui: &mut Ui){
        //TODO: use linux capabilities to add raw socket ability https://squidarth.com/networking/systems/rc/2018/05/28/using-raw-sockets.html
        if nix::unistd::Uid::current().is_root() {
            ui.horizontal( |ui| {
                ui.label("ARP Mode");
                ui.radio_value(&mut self.arp_mode, core::network::arp::ArpModes::Passive, "Passive").on_hover_text("Just listen for ARP requests");
                ui.radio_value(&mut self.arp_mode, core::network::arp::ArpModes::Active, "Active").on_hover_text("Send ARP requests to all hosts on the network");
                if ui.button("Start").clicked() {
                    println!("ARP Mode: {:?}", self.arp_mode);
                    ctx.set_pixels_per_point(2.0); 
                }
            });
            ui.end_row();
        }
        else {
            ui.label("This feature requires root privileges because it uses raw sockets.");
            ui.hyperlink_to("More info.", "https://squidarth.com/networking/systems/rc/2018/05/28/using-raw-sockets.html");
        }
}


    fn state2(&mut self, ctx: &Context, ui: &mut Ui){
        ui.label("this is the menu 2");
    }

    fn state3(&mut self, ctx: &Context, ui: &mut Ui){
        ui.label("this is the menu 3 ");
    }

    fn state4(&mut self, ctx: &Context, ui: &mut Ui){
        ui.label("this is the menu 4");
    }

}


