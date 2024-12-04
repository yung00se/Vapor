use eframe::egui::{self, Color32, Sense, Vec2, FontId, ScrollArea};
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
                ScrollArea::vertical().show(ui, |ui| {
                    for user in leaderboard.iter() {
                        ui.horizontal(|ui| {
                            ui.add_space(ui.available_width() / 2.0 - 150.0);
                            ui.label(user.Username.clone());
                            ui.add_space(100.0);
                            
                            ui.label(format!("High Score: {}", user.HighScore.to_string()).as_str());
                        });
                    }
                });
            });
        //} else { &mut self.populate_leaderboard();}
    }
}