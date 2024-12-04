use eframe::{App, Frame};
use eframe::egui;
use crate::pages::game_hub::{self, GameIcon};
use crate::data_base_api::{DbAPI, MakeRequest};
//use vapor::user_info::User;
//use vapor::pages::navigator::NavBar;
//use vapor::pages::login_page::DisplayLanding;
use crate::pages::game_hub::DisplayLibrary;
use crate::pages::friends_page::DisplayFriends;
use crate::pages::leaderboard_page::DisplayLeaderboard;
use eframe::egui::{Label, RichText, Sense, TextEdit, TextStyle, Align};
use eframe::egui::{TopBottomPanel, CentralPanel, Color32, Key, Button};
               
pub struct Vapor {
    pub username: String,
    password: String,
    id: i32,
    pub db_api: DbAPI,
    current_page: String,
    label_text: String,
    pub game_library: Vec<GameIcon>,
    pub add_friend_input: String,
}

impl Default for Vapor{
    fn default() -> Self{
        Self{
            username: "".to_string(),
            password: "".to_string(),
            id: -1,
            db_api: DbAPI::new(),
            current_page: "land".to_string(),
            label_text: "Login".to_string(),
            game_library: Vec::new(),
            add_friend_input: "".to_string(),
        }
    }
}

impl App for Vapor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        if self.id == -1 {
            self.display_landing(ctx);

            if let Some(user_info) = self.db_api.user.lock().unwrap().pop() {
                self.id = user_info.UserID;
            }
        }
        else {
            self.show_nav_bar(ctx);
        }

        // Draw current page
        self.show_current_page(ctx);
    }
}

impl Vapor {
    fn display_landing(&mut self, ctx: &egui::Context){
        TopBottomPanel::top("login-or-signup").show(ctx, |ui| {
            if self.label_text == "Login" {
                ui.horizontal(|ui| {
                    if ui
                        .add(Label::new(RichText::new("Login").text_style(TextStyle::Heading).color(Color32::from_rgb(0, 200, 200))).sense(Sense::click()))
                        .clicked() { self.label_text = "Login".into() }
    
                        ui.add_space(75.0);
    
                        if ui
                        .add(Label::new(RichText::new("Signup").text_style(TextStyle::Heading)).sense(Sense::click()))
                        .clicked() { self.label_text = "Sign Up".into()  } /*End Login/Signup Buttons*/ });
            }
            else {
                ui.horizontal(|ui| {
                    if ui
                        .add(Label::new(RichText::new("Login").text_style(TextStyle::Heading)).sense(Sense::click()))
                        .clicked() { self.label_text = "Login".into() }

                        ui.add_space(75.0);

                        if ui
                        .add(Label::new(RichText::new("Signup").text_style(TextStyle::Heading).color(Color32::from_rgb(0, 150, 200))).sense(Sense::click()))
                        .clicked() { self.label_text = "Sign Up".into()  } /*End Login/Signup Buttons*/ });
            }
        });
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered( |ui| {
                ui.add_space(150.0);
                ui.heading(self.label_text.clone());

                ui.add_space(50.0);
            
                ui.label("Username:");
                ui.add(
                    TextEdit::singleline(&mut self.username)
                        .desired_width(200.0)
                        .horizontal_align(Align::Center)
                );

                
                ui.add_space(10.0);
                ui.label("Password:");
                ui.add(
                    TextEdit::singleline(&mut self.password)
                        .desired_width(200.0)
                        .horizontal_align(Align::Center)
                        .password(true)
                );
            

                ui.add_space(20.0);
                //ui.label(RichText::new("Test").color(Color32::RED));
                let button = ui.add(Button::new(self.label_text.clone()));
                if ui.input(|i| i.key_pressed(Key::Enter)) 
                    || button.clicked() { 
                        if self.label_text.clone() == "Login" {
                            println!("im here");
                            self.request_login();
                        }
                        else {self.request_signup();}
                }
            });

            });
    }

    fn show_nav_bar(&mut self, ctx: &egui::Context){
        egui::TopBottomPanel::top("page-directory").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let games_button = ui.add(Button::new("Games"));
                ui.add_space(75.0);
                let friends_button = ui.add(Button::new("Friends"));
                ui.add_space(75.0);
                let leaderboard_button = ui.add(Button::new("Leaderboard"));

                if games_button.clicked() {
                    self.current_page = "lib".to_string();
                }

                if friends_button.clicked() {
                        self.current_page = "friends".to_string();

                        self.db_api.get_friends_list(self.id.to_string().as_str());

                        eprint!("Got friends list");
                }

                if leaderboard_button.clicked() {
                        self.current_page ="leaderboards".to_string();
                        // Make get call here
                        self.db_api.get_leaderboard();
                }
            });
        });
    }

    pub fn show_current_page(&mut self, ctx: &egui::Context){
        if self.current_page == "lib" {self.display_library(ctx)}
        else if self.current_page == "friends" {self.display_friends(ctx)}
        else if self.current_page == "leaderboards" {self.display_leaderboard(ctx);}
    }
}
        

impl Vapor {
    fn request_login(&mut self){
        // make signup request here
        //let url = format!("https://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net/api/User/LookForUser?username={}", self.user.name);
        self.db_api.get_login(self.username.as_str());
    }


    fn request_signup(&mut self){
        //let url = format!("https://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net/api/User/AddUser?username={}&password={}", self.user.name, self.user.password);
        self.db_api.post_signup(self.username.as_str(), self.password.as_str());
    }
}

#[tokio::main]
async fn main() {
    let native_options = eframe::NativeOptions::default();

    let _ = eframe::run_native( // Start Vapor
        "Word Unscrambler", // Set the app title
        native_options, 
        Box::new(|_cc| Ok(Box::new(Vapor::default()))),
    );
}
