use crate::appui;
use crate::appui::menuitems::View;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct OffToolboxApp {
    // Example stuff:
    #[serde(skip)] // opted out of serialization
    version: &'static str,
    //#[serde(skip)] // opted out of serialization
    state: OffToolboxState,
    networkmenustate: appui::menuitems::networkmenu::NetworkMenu,
}

impl Default for OffToolboxApp {
    fn default() -> Self {
        Self {
            version: "0.0.1",
            state: OffToolboxState::Main,
            networkmenustate: appui::menuitems::networkmenu::NetworkMenu::default(),
        }
    }
}



#[derive(serde::Deserialize, serde::Serialize)]
pub enum OffToolboxState {
    Main,
    Network,
    Settings,
    About,
    Quit,
}

impl OffToolboxApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}
impl eframe::App for OffToolboxApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut open = false;
        egui::Window::new("OffToolbox").open(&mut open).show(ctx, |ui| {
            ui.heading("OffToolbox test");
        });
        
        egui::SidePanel::left("side_panel").resizable(false).show(ctx, |ui| {
            ui.heading("OffToolbox");
            ui.label("The offensive toolbox.");
            ui.label(format!("v{}", self.version));
            ui.separator();
            sidemenu(ui, &mut self.state);

            ui.separator();

        });
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

            });
        });
        egui::Window::new("Quit Menu")
            .collapsible(false)
            .resizable(false)
            .hscroll(false)
            .open(&mut open)
            .show(ctx, |ui| {
                ui.label("Are you sure you want to quit?");
                if ui.button("Yes").clicked() {
                    std::process::exit(0);
                }
                if ui.button("No").clicked() {
                }
            });


        egui::CentralPanel::default().show(ctx, |ui| {

            match self.state {
                OffToolboxState::Main => {
                    appui::menuitems::mainmenu::default(ctx, ui);
                    powered_by_egui_and_eframe(ui);
                }
                OffToolboxState::Network => {
                    appui::menuitems::networkmenu::NetworkMenu::ui(&mut self.networkmenustate,ctx,ui);
                }
                OffToolboxState::Settings => {
                    appui::menuitems::settingsmenu::default(ctx, ui);
                }
                OffToolboxState::About => {
                    appui::menuitems::aboutmenu::default(ui);
                }
                OffToolboxState::Quit => {
                    appui::menuitems::quitmenu::default(ctx, ui);
                }
            }

            ui.separator();


            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

fn sidemenu(ui: &mut egui::Ui, state: &mut OffToolboxState) {
        ui.vertical_centered_justified(|ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {

                if ui.button("App")
                    .on_hover_ui(|ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.label("This is a tooltip");
                        });
                    }).clicked() {
                        *state = OffToolboxState::Main;
                    }


                if ui.button("Network").clicked() {
                    *state = OffToolboxState::Network;
                }


                if ui.button("Settings").clicked() {
                    *state = OffToolboxState::Settings;
                }


                if ui.button("About").clicked() {
                    *state = OffToolboxState::About;
                }


                if ui.button("Quit").clicked() {
                    *state = OffToolboxState::Quit;
                }
            });
        });
    }
