use eframe::egui::{self, Color32, FontId, Sense, CentralPanel, SidePanel, Label, Key, TextEdit};
use eframe::emath::Vec2;
use eframe::glow::FRAGMENT_INTERPOLATION_OFFSET_BITS;
use crate::data_base_api::MakeRequest;
//use crate::user_info::User;
use crate::vapor::Vapor;

pub trait DisplayFriends{
    fn display_friends(&mut self, ctx: &egui::Context);
}

impl DisplayFriends for Vapor {
    fn display_friends(&mut self, ctx: &egui::Context){
        SidePanel::left("add-friend").show(ctx, |ui| {
            //ui.horizontal(|ui| {
                ui.label("Add Friend");
                ui.add(TextEdit::singleline(&mut self.add_friend_input));
                if ui.input(|i| i.key_pressed(Key::Enter)) { 
                    self.db_api.add_friend(self.username.as_str(), self.add_friend_input.clone().as_str());
                }
            //});       
        });
        let friends_list = &self.db_api.friends_list.lock().unwrap();
        SidePanel::right("friends-list").show(ctx, |ui| {
            for friend in friends_list.iter() {
                let (friend_rect, _response) = ui.allocate_exact_size(Vec2::new(ctx.available_rect().right(), 50.0), Sense::click());
                
                ui.painter().rect_filled(
                    friend_rect,
                    0.0,
                    Color32::from_rgb(248, 248, 248));
                
                ui.painter().text( friend_rect.center(),
                                    egui::Align2::LEFT_CENTER,
                                    friend.clone(), //+ format!("\t\t\tHigh Score: {}", friend.HighScore.to_string().as_str()).as_str(),
                                    FontId::default(),
                                    Color32::from_rgb(40, 40, 40));}                    
        });
    }
}
