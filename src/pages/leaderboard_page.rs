use eframe::egui::{self, Color32, Sense, Vec2, FontId};
//use crate::user_info::User;
use crate::vapor::Vapor;

pub trait DisplayLeaderboard{
    fn display_leaderboard(&mut self, ctx: &egui::Context);
}

pub trait GetLeaderboard {
    fn get_leaderboard(self);
}

impl DisplayLeaderboard for Vapor {
    fn display_leaderboard(&mut self, ctx: &egui::Context) {
        let leaderboard = &self.db_api.leaderboard.lock().unwrap();
            egui::CentralPanel::default().show(ctx, |ui| {
                // leaderboard is empty here for some reason
                for user in leaderboard.iter() {
                    let (user_rect, _response) = ui.allocate_exact_size(Vec2::new(200.0, 15.0), Sense::hover());

                    ui.painter().rect_filled(
                        user_rect,
                        0.0,
                        Color32::from_rgb(248, 248, 248));

                    ui.painter().text(
                        user_rect.center(),
                        egui::Align2::LEFT_CENTER,
                        user.Username.clone() + format!("\t\t\t\t\t\t\t\t\t\t\tHigh Score: {}", user.UserID.to_string().as_str()).as_str(),
                        FontId::default(),
                        Color32::from_rgb(40, 40, 40));
                }
            });
        //} else { &mut self.populate_leaderboard();}
    }
}