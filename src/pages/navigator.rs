use eframe::egui::{self, Label, Sense};
use crate::{data_base_api::{DbAPI, MakeRequest}, user_info::User};
use std::sync::{Arc, Mutex};
use crate::vapor::Vapor;

pub trait NavBar{
    fn show_nav_bar(&mut self, ctx: &egui::Context);
}

impl NavBar for Vapor{
    fn show_nav_bar(&mut self, ctx: &egui::Context){
        egui::TopBottomPanel::top("page-directory").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .add(Label::new("Games").sense(Sense::click()))
                    .clicked() { 
                        self.current_page = "lib".to_string();
                }

                ui.add_space(75.0);

                if ui
                    .add(Label::new("Friends").sense(Sense::click()))
                    .clicked() {
                        self.db_api.get_friends_list(self.current_user.id.to_string().as_str()); // Get friends list for the current user
                        self.db_api.get_user_list();                        
                        self.current_page = "friends".to_string();

                        eprint!("Got friends list");
                }

                ui.add_space(75.0);

                if ui
                    .add(Label::new("Leaderboards").sense(Sense::click()))
                    .clicked() {
                        self.current_page ="leaderboards".to_string();
                        let db_api = DbAPI::new();
                        // Make get call here
                        db_api.get_leaderboard();

                        // let first_user= db_api.leaderboard.lock().unwrap().pop();
                }/*End Page Directory*/});
        });
    }

}
