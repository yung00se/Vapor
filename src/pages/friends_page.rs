use eframe::egui::{self, Color32, FontId, Sense};
use eframe::emath::Vec2;
use crate::user_info::User;

pub trait DisplayFriends{
    fn display_friends(&mut self, ctx: &egui::Context);
}

impl DisplayFriends for User{
    fn display_friends(&mut self, ctx: &egui::Context){
        if let Some(friends_list) = &self.friends{
            egui::SidePanel::right("friends-list").show(ctx, |ui| {
                for (friend, score) in friends_list.iter() {
                    let (friend_rect, _response) = ui.allocate_exact_size(Vec2::new(ctx.available_rect().right(), 50.0), Sense::click());
                    
                    ui.painter().rect_filled(
                        friend_rect,
                        0.0,
                        Color32::from_rgb(248, 248, 248));
                    
                    ui.painter().text( friend_rect.center(),
                                       egui::Align2::LEFT_CENTER,
                                       friend.name.clone() + format!("\t\t\tHigh Score: {}", score.to_string().as_str()).as_str(),
                                       FontId::default(),
                                       Color32::from_rgb(40, 40, 40));}                    
            });
            
        } else { &mut self.populate_friends(); }
    }
}


