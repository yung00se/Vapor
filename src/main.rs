//use tokio::{self, runtime::Runtime};

use std::default;

//use api::{ WordApi, MakeRequest};
use eframe::egui::{Event, FontFamily, FontId, FontSelection, RichText, TextEdit};
use eframe::{App, Frame};
use eframe::egui::{self, CentralPanel, Label, Sense, Color32, Context, text::Fonts, FontDefinitions, Key, Painter, Pos2, Rect, Rounding, Shape, SidePanel, Stroke, TopBottomPanel, Vec2};
use eframe::egui::epaint::{RectShape};
use vapor::game_hub::GameHub;
use vapor::friends_page::FriendsPage;
use vapor::leaderboard_page::LeaderboardPage;
use vapor::login_page::LoginPage;
use vapor::signup_page::SignupPage;
use std::process::{Command};
//use vapor::login_window::{LoginWindow};
/*
use emath::Align2;
use shape_builder::{ShapeAttributes, RoundingType, Dimensions};
use ui_elements::{guess_boxes, letter_square, GenerateAnchors, GenerateUiShapes, UiElements};
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use crate::game_state::UpdateGameVariables;
use regex::Regex;
*/
enum Page {
    LoginPage(LoginPage),
    SignupPage(SignupPage),
    GameHub(GameHub),
    FriendsPage(FriendsPage),
    LeaderboardPage(LeaderboardPage),
}

struct Vapor {
    current_page: Page,
    page_index: i32,
    logged_in: bool,
    username: String,
    password: String,
}

impl Default for Vapor {
    fn default() -> Self {
        Self {
            current_page: Page::LoginPage(LoginPage::default()),
            page_index: 0,
            logged_in: false,
            username: String::from(""),
            password: String::from(""),
        }
    }
}

impl App for Vapor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        if !self.logged_in {
            CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui
                        .add(Label::new("Login").sense(Sense::click()))
                        .clicked()
                        && self.page_index != 0 {
                            self.current_page = Page::LoginPage(LoginPage::default());
                            self.page_index = 0;
                        }
                    ui.add_space(75.0);
                    if ui
                        .add(Label::new("Signup").sense(Sense::click()))
                        .clicked()
                        && self.page_index != 1 {
                            self.current_page = Page::SignupPage(SignupPage::default());
                            self.page_index = 1;
                        }
                });
            
                match &mut self.current_page {
                    Page::SignupPage(page) => {
                        self.username = page.username.clone();
                        self.password = page.password.clone();
                        ui.heading("Signup");
                        ui.label("Username:");
                        ui.add(TextEdit::singleline(&mut page.username));
                        ui.label("Password:");
                        ui.add(TextEdit::singleline(&mut page.password).password(true));
                        ui.label(RichText::new(page.error_msg.clone()).color(Color32::RED));
            
                        if ui.input(|i| i.key_pressed(Key::Enter)) {
                            // if signup is successful
                            if page.request_signup() {
                                self.current_page = Page::GameHub(GameHub::default());
                                self.logged_in = true;
                            }
                            else {
                                println!("Failed sign up");
                                page.error_msg = "The username and password do not match".into();
                            }
                        }
                    }
                    Page::LoginPage(page) => {
                        self.username = page.username.clone();
                        self.password = page.password.clone();
                        ui.heading("Login");
                        ui.label("Username:");
                        ui.add(TextEdit::singleline(&mut page.username));
                        ui.label("Password:");
                        ui.add(TextEdit::singleline(&mut page.password).password(true));
                        ui.label(RichText::new(page.error_msg.clone()).color(Color32::RED));
            
                        if ui.input(|i| i.key_pressed(Key::Enter)) {
                            if page.request_login() {
                                self.current_page = Page::GameHub(GameHub::default());
                                self.logged_in = true;
                            }
                            else {
                                println!("Failed sign up");
                                page.error_msg = "The username and password do not match".into();
                            }
                        }
                    }
                    _ => ()
                }
            });
        }

        else {
            CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui
                        .add(Label::new("Games").sense(Sense::click()))
                        .clicked()
                        && self.page_index != 0 {
                            self.current_page = Page::GameHub(GameHub::default());
                            self.page_index = 0;
                        }
                    ui.add_space(75.0);
                    if ui
                        .add(Label::new("Friends").sense(Sense::click()))
                        .clicked()
                        && self.page_index != 1 {
                            self.current_page = Page::FriendsPage(FriendsPage::default());
                            self.page_index = 1;
                        }
                    ui.add_space(75.0);
                    if ui
                        .add(Label::new("Leaderboards").sense(Sense::click()))
                        .clicked()
                        && self.page_index != 2 {
                            self.current_page = Page::LeaderboardPage(LeaderboardPage::default());
                            self.page_index = 2;
                        }
                });
                match &mut self.current_page {
                    Page::GameHub(page) => {
                        //println!("On Gamehub page");
                        for game in page.games.iter() {
                            //ui.painter().add(game.rect);
                            //println!(game.rect)
                            let (game_rect, response) = ui.allocate_exact_size(Vec2::new(200.0, 50.0), Sense::click());
                            ui.painter().rect_filled(game_rect, 0.0, Color32::BLACK);
                            ui.painter().text(
                                game_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                game.title.clone(),
                                FontId::default(),
                                Color32::WHITE,
                            );
                            if response.clicked() {
                                game.handle_click();
                            }
                        }
                        
                    }
                    Page::FriendsPage(page) => {
                        //println!("On Friends page");
                        //let line = ui.allocate_exact_size(Vec2::new(600.0, 2.0), Sense::hover());
                        for friend in page.friends.iter() {
                            let (friend_rect, _response) = ui.allocate_exact_size(Vec2::new(200.0, 20.0), Sense::hover());

                            ui.painter().rect_filled(friend_rect, 0.0, Color32::from_rgb(248, 248, 248));
                            //println!("{}", friend.highscore);
                            ui.painter().text(
                                friend_rect.center(),
                                egui::Align2::LEFT_CENTER,
                                friend.username.clone() + format!("\t\t\tHigh Score: {}", friend.highscore.to_string().as_str()).as_str(),
                                FontId::default(),
                                Color32::from_rgb(40, 40, 40),
                            );
                        }
                    }
                    Page::LeaderboardPage(page) => {
                        //println!("On Leaderboard page");
                        for user in page.users.iter() {
                            let (user_rect, _response) = ui.allocate_exact_size(Vec2::new(200.0, 15.0), Sense::hover());
                            ui.painter().rect_filled(user_rect, 0.0, Color32::from_rgb(248, 248, 248));
                            ui.painter().text(
                                user_rect.center(),
                                egui::Align2::LEFT_CENTER,
                                user.username.clone() + format!("\t\t\t\t\t\t\t\t\t\t\tHigh Score: {}", user.highscore.to_string().as_str()).as_str(),
                                FontId::default(),
                                Color32::from_rgb(40, 40, 40),
                            );
                        }
                    }
                    _ => ()
                }
            });
        }
    }
}

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    let native_options = eframe::NativeOptions::default(); // Create default native options
    /* 
    // initialize game hub attributes
    let mut game_hub = GameHub::default();
    game_hub
        .add_page(String::from("Games"), 1)
        .add_page(String::from("Friends"), 2)
        .add_page(String::from("Leaderboard"), 3)
        .add_game(100.0, 100.0, String::from("Word Scramble"), 4)
        .add_game(100.0, 100.0, String::from("Other Game"), 5);
    */
    let _ = eframe::run_native( // Run the native app
        "Word Unscrambler", // Set the app title
        native_options, // Set the native options
        //Box::new(|_cc| Ok(Box::new(LoginWindow::default()))), // Create a new WordUnscramblerApp instance
        Box::new(|_cc| Ok(Box::new(Vapor::default()))), // Create a new WordUnscramblerApp instance
    );
}