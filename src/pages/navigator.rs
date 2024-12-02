use eframe::egui::{self, Label, Sense};
use crate::{data_base_api::{DbAPI, MakeRequest}, user_info::User};
use std::sync::{Arc, Mutex};

pub trait NavBar{
    fn show_nav_bar(&mut self, ctx: &egui::Context);
}

impl NavBar for User{
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
                        self.current_page = "friends".to_string();
                        let db_api = DbAPI::new();

                        db_api.get_friends_list(self.id.to_string().as_str());
                        // set the users friends list

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
                        let first_user= *db_api.leaderboard_arc.lock().unwrap();
                        /* 
                        let mut populated = false;
                        match &self.leaderboard {
                            Some(leaderboard) => {
                                if leaderboard.is_empty() {
                                    eprint!("leaderboard is empty\n");
                                    populated = true;
                                }
                                else {
                                    eprint!("leaderboard has entries\n");
                                    populated = true;
                                }
                            }
                            None => {
                                eprint!("Leaderboard is None\n");
                            }
                        }
                        */
                }/*End Page Directory*/});
        });
    }

}
