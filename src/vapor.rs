use eframe::{App, Frame,
             egui::{self, Label, RichText, Sense, TextEdit, TextStyle, Align, TopBottomPanel, CentralPanel, Color32, Key, Button}};
use crate::data_base_api::{DbAPI, MakeRequest};
use crate::user_info::User;
use crate::pages::{navigator::NavBar, game_hub::{DisplayLibrary, GameIcon}, friends_page::DisplayFriends, leaderboard_page::DisplayLeaderboard};

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
            current_page: "land".to_string(),
            game_library: Vec::new(),
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

            // Draw current page
            self.show_current_page(ctx);
        }
    }
}

impl Vapor {

fn display_landing(&mut self, ctx: &egui::Context){

    let mut label_text: String = "Login".into();

    TopBottomPanel::top("login-or-signup").show(ctx, |ui| { //Login or  Signup Selection
        ui.horizontal(|ui| {

            if ui
                .add(Label::new(RichText::new("Login").text_style(TextStyle::Heading).color(Color32::from_rgb(0, 200, 200))).sense(Sense::click()))
                .clicked() { label_text = "Login".into() }
 
           ui.add_space(75.0);

            if ui
                .add(Label::new(RichText::new("Signup").text_style(TextStyle::Heading)).sense(Sense::click()))
                .clicked() { label_text = "Sign Up".into()  } /*End Login/Signup Buttons*/ });
    });

    CentralPanel::default().show(ctx, |ui| { //Username/Password entry fields
        ui.vertical_centered( |ui| {
            ui.add_space(150.0);
            ui.heading(&label_text);

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
            let button = ui.add(Button::new(&label_text));
            if ui.input(|i| i.key_pressed(Key::Enter)) 
                || button.clicked() { 
                    if &label_text == "Login" {
                        println!("im here");
                        self.request_login();
                    }
                    else {self.request_signup();}
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
        self.display_friends(ctx);
        self.display_users(ctx)}
    else if self.current_page == "leaderboards" {self.display_leaderboard(ctx);}
}

} //End Vapor Implementation
