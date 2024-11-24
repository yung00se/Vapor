use eframe::egui::{Event, FontFamily, FontId, FontSelection, RichText, TextEdit};
use eframe::{App, Frame};
use eframe::egui::{self, CentralPanel, Label, Sense, Color32, Context, text::Fonts, FontDefinitions, Key, Painter, Pos2, Rect, Rounding, Shape, SidePanel, Stroke, TopBottomPanel, Vec2};
use vapor::game_hub::GameHub;
use vapor::pages::{friends_page::FriendsPage, leaderboard_page::LeaderboardPage, login_page::LoginPage, signup_page::SignupPage};




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
                        && self.page_index != 1 { self.current_page = Page::SignupPage(SignupPage::default());
                                                  self.page_index = 1}
                });
            
                match &mut self.current_page {
                    Page::SignupPage(page) => {
                        self.username = page.username.clone();
                        self.password = page.password.clone();
                        ui.heading("Signup");

                        ui.label("Username:");
                        ui.add(TextEdit::singleline(&mut page.username)); //Username Entry

                        ui.label("Password:");
                        ui.add(TextEdit::singleline(&mut page.password).password(true)); //Password Entry
                        
                        ui.label(RichText::new(page.error_msg.clone()).color(Color32::RED)); //Error Message 
            
                        if ui.input(|i| i.key_pressed(Key::Enter)) {

                            if page.request_signup() { self.current_page = Page::GameHub(GameHub::default());
                                                       self.logged_in = true } //Successful Signup

                            else { println!("Failed sign up");
                                   page.error_msg = "The username and password do not match".into() } //Signup Error

                        }
                    } //End Signup Page

                    Page::LoginPage(page) => {
                        self.username = page.username.clone();
                        self.password = page.password.clone();
                        ui.heading("Login");
                        
                        ui.label("Username:");
                        ui.add(TextEdit::singleline(&mut page.username)); //Username Field
                        
                        ui.label("Password:");
                        ui.add(TextEdit::singleline(&mut page.password).password(true)); //Password Field
                        
                        ui.label(RichText::new(page.error_msg.clone()).color(Color32::RED)); //Error Message
            
                        if ui.input(|i| i.key_pressed(Key::Enter)) {
                            if page.request_login() { self.current_page = Page::GameHub(GameHub::default());
                                                      self.logged_in = true } //Successful Login

                            else { println!("User Authentication Failed...");
                                   page.error_msg = "The username and password do not match".into() } //Authentication Error
                        }
                    } //End Login Page

                    _ => () //Other Pages (i.e., Gamehub, Friends Page, High Score Page)
                }});

        } //End Authentication/Signup Pages//

        else {
            TopBottomPanel::top("page-directory").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui
                        .add(Label::new("Games").sense(Sense::click()))
                        .clicked()
                        && self.page_index != 0 { self.current_page = Page::GameHub(GameHub::default());
                                                  self.page_index = 0 }

                    ui.add_space(75.0);

                    if ui
                        .add(Label::new("Friends").sense(Sense::click()))
                        .clicked()
                        && self.page_index != 1 { self.current_page = Page::FriendsPage(FriendsPage::default());
                                                  self.page_index = 1 }

                    ui.add_space(75.0);

                    if ui
                        .add(Label::new("Leaderboards").sense(Sense::click()))
                        .clicked()
                        && self.page_index != 2 { self.current_page = Page::LeaderboardPage(LeaderboardPage::default());
                                                  self.page_index = 2 }
                }); // End Page Directory
            });
            match &self.current_page {
                Page::FriendsPage(page) => {
                    page.friend_page_home(ctx);
                },
                Page::LeaderboardPage(page) => {
                    page.show_leaderboards(ctx);
                },
                Page::GameHub(page) => {
                    page.show_game_hub(ctx);
                },
                _ => (),
            }
        }
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();

    let _ = eframe::run_native( // Start Vapor
        "Word Unscrambler", // Set the app title
        native_options, 
        Box::new(|_cc| Ok(Box::new(Vapor::default()))),
    );
}
