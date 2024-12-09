use eframe::egui::{self, mutex::MutexGuard, Align, CentralPanel, Color32, FontId, Grid, Label, Layout, RichText, ScrollArea, Sense, TextStyle, TopBottomPanel, Vec2};
use ::egui::plot::Text;
//use crate::user_info::User;
use crate::{data_base_api::{DbAPI, UserEntry}, vapor::Vapor};
use std::sync::Mutex;

pub trait DisplayLeaderboard{
    fn display_leaderboard(&mut self, ctx: &egui::Context);
}

pub struct Leaderboard {
    current_page: String,
    pub db_api: DbAPI,
}

impl Default for Leaderboard {
    fn default() -> Self {
        Self {
            current_page: "Word Scramble".into(),
            db_api: DbAPI::new(),
        }
    }
}

impl Leaderboard {
    pub fn display_leaderboard(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("leaderboard-tabs").show(ctx, |ui| { //Login or  Signup Selection
            ui.horizontal(|ui| {
                if ui
                    .add(Label::new(RichText::new("Word Scramble").text_style(TextStyle::Heading).color(Color32::from_rgb(0, 200, 200))).sense(Sense::click()))
                    .clicked() { self.current_page = "Word Scramble".into() }
                
                ui.add_space(75.0);

                if ui
                    .add(Label::new(RichText::new("Sudoku").text_style(TextStyle::Heading)).sense(Sense::click()))
                    .clicked() { self.current_page = "Sudoku".into()  } /*End Login/Signup Buttons*/ });

                ui.add_space(75.0);

                if ui
                    .add(Label::new(RichText::new("Rapid Math").text_style(TextStyle::Heading)).sense(Sense::click()))
                    .clicked() { self.current_page = "Rapid Math".into() }
                println!("HELLO");
        });
        let ws_leaderboard = &self.db_api.leaderboard.lock().unwrap();
        let s_leaderboard = &self.db_api.sudoku_leaderboard.lock().unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut index = 0;
            if self.current_page == "Word Scramble".to_string() {
                ui.horizontal(|ui| {
                    ui.add_space(200.0);
                    ui.vertical(|ui| {
                        ui.heading("User");
                        ui.add_space(10.0);
                        
                        for user in ws_leaderboard.iter() {
                            if index > 9 {
                                break
                            }
                            ui.label(user.Username.clone());
                            ui.add_space(10.0);
                            index += 1;
                        }
                    });

                    ui.add_space(100.0);
                    ui.vertical(|ui| {
                        ui.heading("High Score");
                        ui.add_space(10.0);
                        index = 0;
                        for user in ws_leaderboard.iter() {
                            if index > 9 {
                                break
                            }
                            ui.label(user.HighScoreWord.to_string());
                            ui.add_space(10.0);
                            index += 1;
                        }
                    });
                });
            }
            else if self.current_page == "Sudoku".to_string(){
                ui.horizontal(|ui| {
                    ui.add_space(200.0);
                    ui.vertical(|ui| {
                        ui.heading("User");
                        ui.add_space(10.0);
                        
                        for user in s_leaderboard.iter() {
                            if index > 9 {
                                break
                            }
                            ui.label(user.Username.clone());
                            ui.add_space(10.0);
                            index += 1;
                        }
                    });

                    ui.add_space(100.0);
                    ui.vertical(|ui| {
                        ui.heading("High Score");
                        ui.add_space(10.0);
                        index = 0;
                        for user in s_leaderboard.iter() {
                            if index > 9 {
                                break
                            }
                            ui.label(user.HighScoreSudoku.to_string());
                            ui.add_space(10.0);
                            index += 1;
                        }
                    });
                });
            }
        });
    }
}

impl DisplayLeaderboard for Vapor {
    fn display_leaderboard(&mut self, ctx: &egui::Context) {
        //eprint!("{}\n", self.current_page);
        TopBottomPanel::top("leaderboard-tabs").show(ctx, |ui| { //Login or  Signup Selection
            ui.horizontal(|ui| {
                if ui
                    .add(Label::new(RichText::new("Word Scramble").text_style(TextStyle::Heading).color(Color32::from_rgb(0, 200, 200))).sense(Sense::click()))
                    .clicked() { self.leaderboard.current_page = "Word Scramble".into() }
                
                ui.add_space(75.0);

                if ui
                    .add(Label::new(RichText::new("Sudoku").text_style(TextStyle::Heading)).sense(Sense::click()))
                    .clicked() { self.leaderboard.current_page = "Sudoku".into()  } /*End Login/Signup Buttons*/
                ui.add_space(75.0);
                
                if ui
                    .add(Label::new(RichText::new("Rapid Math").text_style(TextStyle::Heading)).sense(Sense::click()))
                    .clicked() { 
                        self.leaderboard.current_page = "Rapid Math".into();
                }
            });
        });
        let ws_leaderboard = &self.db_api.leaderboard.lock().unwrap();
        let s_leaderboard = &self.db_api.sudoku_leaderboard.lock().unwrap();
        let m_leaderboard = &self.db_api.math_leaderboard.lock().unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut index = 0;
            if self.leaderboard.current_page == "Word Scramble".to_string() {
                ui.horizontal(|ui| {
                    ui.add_space(200.0);
                    ui.vertical(|ui| {
                        ui.heading("User");
                        ui.add_space(10.0);
                        
                        for user in ws_leaderboard.iter() {
                            if index > 9 {
                                break
                            }
                            ui.label(user.Username.clone());
                            ui.add_space(10.0);
                            index += 1;
                        }
                    });

                    ui.add_space(100.0);
                    ui.vertical(|ui| {
                        ui.heading("High Score");
                        ui.add_space(10.0);
                        index = 0;
                        for user in ws_leaderboard.iter() {
                            if index > 9 {
                                break
                            }
                            ui.label(user.HighScoreWord.to_string());
                            ui.add_space(10.0);
                            index += 1;
                        }
                    });
                });
            }
            else if self.leaderboard.current_page == "Sudoku".to_string(){
                ui.horizontal(|ui| {
                    ui.add_space(200.0);
                    ui.vertical(|ui| {
                        ui.heading("User");
                        ui.add_space(10.0);
                        
                        for user in s_leaderboard.iter() {
                            if index > 9 {
                                break
                            }
                            ui.label(user.Username.clone());
                            ui.add_space(10.0);
                            index += 1;
                        }
                    });

                    ui.add_space(100.0);
                    ui.vertical(|ui| {
                        ui.heading("High Score");
                        ui.add_space(10.0);
                        index = 0;
                        for user in s_leaderboard.iter() {
                            if index > 9 {
                                break
                            }
                            ui.label(user.HighScoreSudoku.to_string());
                            ui.add_space(10.0);
                            index += 1;
                        }
                    });
                });
            }
            else if self.leaderboard.current_page == "Rapid Math".to_string(){
                ui.horizontal(|ui| {
                    ui.add_space(200.0);
                    ui.vertical(|ui| {
                        ui.heading("User");
                        ui.add_space(10.0);
                        
                        for user in m_leaderboard.iter() {
                            if index > 9 {
                                break
                            }
                            ui.label(user.Username.clone());
                            ui.add_space(10.0);
                            index += 1;
                        }
                    });

                    ui.add_space(100.0);
                    ui.vertical(|ui| {
                        ui.heading("High Score");
                        ui.add_space(10.0);
                        index = 0;
                        for user in m_leaderboard.iter() {
                            if index > 9 {
                                break
                            }
                            ui.label(user.HighScoreMath.to_string());
                            ui.add_space(10.0);
                            index += 1;
                        }
                    });
                });
            }
        });
    }
}