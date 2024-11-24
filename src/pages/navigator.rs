use eframe::egui::{self, Label, Sense};
use crate::user_info::User;

pub trait NavBar{
    fn show_nav_bar(&mut self, ctx: &egui::Context);
}

impl NavBar for User{
    fn show_nav_bar(&mut self, ctx: &egui::Context){
        egui::TopBottomPanel::top("page-directory").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .add(Label::new("Games").sense(Sense::click()))
                    .clicked() { self.current_page = "lib".to_string() };

                ui.add_space(75.0);

                if ui
                    .add(Label::new("Friends").sense(Sense::click()))
                    .clicked() { self.current_page = "friends".to_string()};

                ui.add_space(75.0);

                if ui
                    .add(Label::new("Leaderboards").sense(Sense::click()))
                    .clicked() { self.current_page ="leaderboards".to_string() }/*End Page Directory*/});
        });
    }

}

