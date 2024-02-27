use egui::Context;
use egui::Ui;


#[derive(serde::Deserialize, serde::Serialize)]
pub struct CryptoMenu {
    module_state: CryptoMenuState,
}

impl Default for CryptoMenu {
    fn default() -> Self {
        Self {
            module_state: CryptoMenuState::Base64,
        }
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
pub enum CryptoMenuState {
    Base64,
    Hex,
    Caesar,
}


impl super::View for CryptoMenu {
    fn ui(&mut self,ctx: &Context, ui: &mut Ui) {
        if ui.button("Base64").clicked() {
            self.module_state = CryptoMenuState::Base64;
        }

        match self.module_state {
            CryptoMenuState::Base64 => {
                ui.label("Base64");
            }
            CryptoMenuState::Hex => {
                ui.label("Hex");
            }
            CryptoMenuState::Caesar => {
                ui.label("Caesar");
            }
        }

    }
}
