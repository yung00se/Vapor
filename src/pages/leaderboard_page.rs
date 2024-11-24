use eframe::egui::{self, Color32, Sense, Vec2, FontId};
use crate::user_info::User;

pub trait DisplayLeaderboard{
    fn display_leaderboard(&mut self, ctx: &egui::Context);
}

impl DisplayLeaderboard for User {
    fn display_leaderboard(&mut self, ctx: &egui::Context) {
        if let Some(leaderboard) = &self.leaderboard.clone() {
            egui::CentralPanel::default().show(ctx, |ui| {
                for user in leaderboard {
                    let (user_rect, _response) = ui.allocate_exact_size(Vec2::new(200.0, 15.0), Sense::hover());

                    ui.painter().rect_filled(
                        user_rect,
                        0.0,
                        Color32::from_rgb(248, 248, 248));

                    ui.painter().text(
                        user_rect.center(),
                        egui::Align2::LEFT_CENTER,
                        user.0.name.clone() + format!("\t\t\t\t\t\t\t\t\t\t\tHigh Score: {}", user.1.to_string().as_str()).as_str(),
                        FontId::default(),
                        Color32::from_rgb(40, 40, 40));
                }
            });
        } else { &mut self.populate_leaderboard();}
    }
}
