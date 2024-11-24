use eframe::egui::{self, Color32, FontId, Label, Sense};
use eframe::emath::Vec2;

#[derive(Clone)]
pub struct Friend {
    pub height: f32,
    pub width: f32,
    pub username: String,
    pub highscore: u16,
}

impl Default for Friend {
    fn default() -> Self {
        Self {
            height: 50.0,
            width: 100.0,
            username: String::from(""),
            highscore: 0,
        }
    }
}

impl Friend {
    fn new(username: String, highscore: u16) -> Self {
        Self {
            height: 50.0,
            width: 100.0,
            username,
            highscore,
        }
    }
}

//Friends Page//
pub struct FriendsPage {
    pub page_name: String,
    pub friends: Vec<Friend>,
}

impl Default for FriendsPage {
    fn default() -> Self {
        Self {
            page_name: String::from("Friends"),
            friends: [ Friend::new(String::from("Paul"), 43),
                       Friend::new(String::from("John"), 26),
                       Friend::new(String::from("Will"), 39),
                       Friend::new(String::from("Spencer"), 38),
                       Friend::new(String::from("Mann"), 46)
                     ].to_vec()
        }
    }
}



impl FriendsPage{
    pub fn friend_page_home(&self, ctx: &egui::Context){
        egui::CentralPanel::default().show(ctx, |ui| {
            for friend in self.friends.iter() {
                let (friend_rect, _response) = ui.allocate_exact_size(Vec2::new(200.0, 50.0), Sense::click());
                
                ui.painter().rect_filled(
                    friend_rect,
                    0.0,
                    Color32::from_rgb(248, 248, 248));
                
                ui.painter().text( friend_rect.center(),
                                   egui::Align2::LEFT_CENTER,
                                   friend.username.clone() + format!("\t\t\tHigh Score: {}", friend.highscore.to_string().as_str()).as_str(),
                                   FontId::default(),
                                   Color32::from_rgb(40, 40, 40));                
            }
        });
    }
}
