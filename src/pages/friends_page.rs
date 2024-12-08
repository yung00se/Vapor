use eframe::egui::{self, Color32, FontId, Sense, SidePanel, Key, TextEdit};
use eframe::emath::Vec2;
use emath::Align2;
use crate::vapor::Vapor;
use crate::data_base_api::MakeRequest;

pub trait DisplayFriends{
    fn display_users(&mut self, ctx: &egui::Context);
    fn display_friends(&mut self, ctx: &egui::Context);
    fn add_friends(&mut self, ctx: &egui::Context);
}

impl DisplayFriends for Vapor {
    fn display_users(&mut self, ctx: &egui::Context){
        let user_list = self.db_api.user_list.lock().unwrap();
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for user in user_list.iter() {
                    let (user_rect, user_button) = ui.allocate_exact_size(Vec2::new(300.0, 50.0), Sense::click());
                    let popup_id = ui.make_persistent_id(&user.UserID);
                    ui.painter().rect_filled(
                        user_rect,
                        0.0,
                        Color32::from_rgb(248, 248, 248));
                    
                    ui.painter().text( user_rect.center(),
                                       egui::Align2::LEFT_CENTER,
                                       &user.Username, //+ format!("\t\t\tHigh Score: {}", friend.HighScore.to_string().as_str()).as_str(),
                                       FontId::default(),
                                       Color32::from_rgb(40, 40, 40));

                let above = egui::AboveOrBelow::Above;
                let close_on_unfocus = egui::popup::PopupCloseBehavior::CloseOnClickOutside;

                egui::popup::popup_above_or_below_widget(ui, popup_id, &user_button, above, close_on_unfocus, |ui|{
                    if ui.add(egui::Button::new("Add Friend")).clicked() {
                        self.db_api.add_friend(self.current_user.id, &user.Username);
                    }
                });

                if user_button.clicked{
                    ui.memory_mut(|mem| mem.toggle_popup(popup_id))
                }}
            });
        });
    }

    fn display_friends(&mut self, ctx: &egui::Context){
       egui::Window::new("Friends")
            .constrain(false)
            .pivot(Align2::RIGHT_BOTTOM)
            .anchor(Align2::RIGHT_BOTTOM, [-10.0, 0.0])
            .show(ctx, |ui| {
                let friends_list = self.db_api.friends_list.lock().unwrap();
                for friend in friends_list.iter()
                { let (friend_rect, _response) = ui.allocate_exact_size(Vec2::new(200.0, 20.0), Sense::click());
                  ui.painter().rect_filled(
                      friend_rect,
                      0.0,
                      Color32::from_rgb(248, 248, 248));
                  
                  ui.painter().text( friend_rect.center(),
                                     egui::Align2::LEFT_CENTER,
                                     &friend, //+ format!("\t\t\tHigh Score: {}", friend.HighScore.to_string().as_str()).as_str(),
                                     FontId::default(),
                                     Color32::from_rgb(40, 40, 40));}
            });
    }
    fn add_friends(&mut self, ctx: &egui::Context){
        SidePanel::left("add-friend").show(ctx, |ui| {
            //ui.horizontal(|ui| {
                ui.label("Add Friend");
                ui.add(TextEdit::singleline(&mut self.add_friend_input));
                if ui.input(|i| i.key_pressed(Key::Enter)) { 
                    self.db_api.add_friend(self.current_user.id, self.add_friend_input.clone().as_str());
                }
        });
    }
}
