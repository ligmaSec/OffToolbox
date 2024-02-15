use egui::Ui;
use crate::core::testt;
pub fn default(ui: &mut Ui) {

   Ui::label(ui,"Please do not use this program for malicious purposes. This program is intended for educational purposes only. I am not responsible for any damage caused by this program. Use at your own risk.");
   testt::testt(); 

}
