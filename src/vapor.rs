use std::time::Duration;

use eframe::{App, Frame,
             egui::{self, Label, RichText, Sense, TextEdit, TextStyle, Align, TopBottomPanel, CentralPanel, Color32, Key, Button}};
use crate::data_base_api::{DbAPI, MakeRequest};
use crate::user_info::User;
use crate::pages::{navigator::NavBar, game_hub::{DisplayLibrary, GameIcon, build_library}, friends_page::DisplayFriends, leaderboard_page::DisplayLeaderboard};

pub struct Vapor {
    pub current_user: User,
    pub db_api: DbAPI,
    pub current_page: String,
    pub game_library: Vec<GameIcon>,
    pub add_friend_input: String,
}

impl Default for Vapor{
    fn default() -> Self{
        Self{
            current_user: User::new("".into(), "".into(), -1),
            db_api: DbAPI::new(),
            current_page: "login".to_string(),
            game_library: build_library(),
            add_friend_input: "".to_string(),
        }
    }
}

impl App for Vapor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        if self.current_user.id == -1 {
            self.display_landing(ctx);
            if let Some(user_info) = self.db_api.user.lock().unwrap().pop() {self.current_user.id = user_info.UserID}
        } else {
            self.show_nav_bar(ctx);
            self.display_friends(ctx);

            if *self.db_api.update_indicator.lock().unwrap() == true {
                self.db_api.get_friends_list(self.current_user.id);
                *self.db_api.update_indicator.lock().unwrap() = false;
            }
        }
        //Draw the current page
        self.show_current_page(ctx);
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}





impl Vapor {

    fn display_landing(&mut self, ctx: &egui::Context){
        TopBottomPanel::top("login-or-signup").show(ctx, |ui| { //Login or  Signup Selection
            ui.horizontal(|ui| {

                if ui
                    .add(Label::new(RichText::new("Log In").text_style(TextStyle::Heading).color(Color32::from_rgb(0, 200, 200))).sense(Sense::click()))
                    .clicked() { self.current_page = "login".into() }
                
                ui.add_space(75.0);

                if ui
                    .add(Label::new(RichText::new("Signup").text_style(TextStyle::Heading)).sense(Sense::click()))
                    .clicked() { self.current_page = "signup".into()  } /*End Login/Signup Buttons*/ });
        });
    }

    fn display_login(&mut self, ctx: &egui::Context){
        CentralPanel::default().show(ctx, |ui| { //Username/Password entry fields
            ui.vertical_centered( |ui| {
                ui.add_space(150.0);
                ui.heading("Log In");

                ui.add_space(50.0);

                ui.label("Username:");
                ui.add(TextEdit::singleline(&mut self.current_user.name)
                       .desired_width(200.0)
                       .horizontal_align(Align::Center));

                ui.add_space(10.0);

                ui.label("Password:");
                ui.add(TextEdit::singleline(&mut self.current_user.password)
                       .desired_width(200.0)
                       .horizontal_align(Align::Center)
                       .password(true));


                ui.add_space(20.0);
                //ui.label(RichText::new("Test").color(Color32::RED));
                let button = ui.add(Button::new("Log In"));
                if ui.input(|i| i.key_pressed(Key::Enter)) 
                    || button.clicked() { self.request_login()}

                if self.current_user.id != -1{
                    self.current_page = "lib".into();
                    ctx.request_repaint();
                }
            });
        });
    }
    
    fn display_signup(&mut self, ctx: &egui::Context){
        CentralPanel::default().show(ctx, |ui| { //Username/Password entry fields
            ui.vertical_centered( |ui| {
                ui.add_space(150.0);
                ui.heading("Sign Up");

                ui.add_space(50.0);

                ui.label("Username:");
                ui.add(TextEdit::singleline(&mut self.current_user.name)
                       .desired_width(200.0)
                       .horizontal_align(Align::Center));

                ui.add_space(10.0);

                ui.label("Password:");
                ui.add(TextEdit::singleline(&mut self.current_user.password)
                       .desired_width(200.0)
                       .horizontal_align(Align::Center)
                       .password(true));


                ui.add_space(20.0);
                //ui.label(RichText::new("Test").color(Color32::RED));
                let button = ui.add(Button::new("Sign Up"));
                if ui.input(|i| i.key_pressed(Key::Enter)) 
                    || button.clicked() { self.request_signup()}

                if self.current_user.id != -1{
                    self.current_page = "lib".into();
                }
            });
        });
    }

    fn request_login(&mut self){
        self.db_api.get_login(self.current_user.name.as_str());
    }

    fn request_signup(&mut self){
        self.db_api.post_signup(self.current_user.name.as_str(), self.current_user.password.as_str());
    }

    pub fn show_current_page(&mut self, ctx: &egui::Context){
        if self.current_page == "lib" {self.display_library(ctx)}
        else if self.current_page == "friends" {
            self.add_friends(ctx);
            self.display_users(ctx)}
        else if self.current_page == "leaderboards" {self.display_leaderboard(ctx);}
        else if self.current_page == "login" {self.display_login(ctx)}
        else if self.current_page == "signup" {self.display_signup(ctx)}
    }

} //End Vapor Implementation
