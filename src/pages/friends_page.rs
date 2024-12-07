use eframe::egui::{self, Color32, FontId, Sense, SidePanel, Key, TextEdit};
use eframe::emath::Vec2;
use crate::vapor::Vapor;
use crate::data_base_api::MakeRequest;

pub trait DisplayFriends{
    fn display_users(&mut self, ctx: &egui::Context);
    fn display_friends(&mut self, ctx: &egui::Context);
}

impl DisplayFriends for Vapor {
    fn display_users(&mut self, ctx: &egui::Context){
        let user_list = self.db_api.user_list.lock().unwrap();
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for user in user_list.iter() {
                    let (user_rect, _response) = ui.allocate_exact_size(Vec2::new(300.0, 50.0), Sense::click());
                    
                    ui.painter().rect_filled(
                        user_rect,
                        0.0,
                        Color32::from_rgb(248, 248, 248));
                    
                    ui.painter().text( user_rect.center(),
                                       egui::Align2::LEFT_CENTER,
                                       &user.Username, //+ format!("\t\t\tHigh Score: {}", friend.HighScore.to_string().as_str()).as_str(),
                                       FontId::default(),
                                       Color32::from_rgb(40, 40, 40));}                 
            });
        });
    }
    fn display_friends(&mut self, ctx: &egui::Context){
        SidePanel::left("add-friend").show(ctx, |ui| {
            //ui.horizontal(|ui| {
                ui.label("Add Friend");
                ui.add(TextEdit::singleline(&mut self.add_friend_input));
                if ui.input(|i| i.key_pressed(Key::Enter)) { 
                    self.db_api.add_friend(self.current_user.name.as_str(), self.add_friend_input.clone().as_str());
                }
        });
    }
}
