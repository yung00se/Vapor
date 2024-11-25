use eframe::{App, Frame};
use eframe::egui;
use vapor::pages::game_hub::DisplayLibrary;
use vapor::data_base_api::{DbAPI, MakeRequest};
use vapor::user_info::User;
use vapor::pages::navigator::NavBar;
//use vapor::pages::login_page::DisplayLanding;
use eframe::egui::{Label, RichText, Sense, TextEdit};
use eframe::egui::{TopBottomPanel, CentralPanel, Color32, Key};
               
pub struct Vapor {
    user: User,
    db_api: DbAPI,
    label_text: String,
}

impl Default for Vapor{
    fn default() -> Self{
        Self{
            user: User::default(),
            db_api: DbAPI::new(),
            label_text: "Login:".to_string(),
        }
    }
}

impl App for Vapor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        match self.user.id {
            Some(id) => {
                //eprint!("User has id {}", id);
                self.user.show_nav_bar(ctx);
            }
            None => {
                self.display_landing(ctx);
                if let Some(user_info) = self.db_api.user.lock().unwrap().pop() {
                    self.user.id = Some(user_info.UserID);
                }
            }
        }
        // Draw current page
        self.user.show_current_page(ctx);
    }
}

pub trait DisplayLanding{
    fn display_landing(&mut self, ctx: &egui::Context);
}

impl DisplayLanding for Vapor {
    fn display_landing(&mut self, ctx: &egui::Context){
        TopBottomPanel::bottom("login-or-signup").show(ctx, |ui| {
            ui.horizontal(|ui| { if ui
                                 .add(Label::new("Login").sense(Sense::click()))
                                 .clicked(){ self.label_text = "Login:".into() }

                                 ui.add_space(75.0);

                                 if ui
                                 .add(Label::new("Signup").sense(Sense::click()))
                                 .clicked() { self.label_text = "Sign Up:".into()  } /*End Login/Signup Buttons*/ });
        });
        CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.label_text.clone());
            
            ui.label("Username:");
            ui.add(TextEdit::singleline(&mut self.user.name)); //Username Entry

            ui.label("Password:");
            ui.add(TextEdit::singleline(&mut self.user.password).password(true));

            ui.label(RichText::new("Test").color(Color32::RED));

            if ui.input(|i| i.key_pressed(Key::Enter)) { 
                if self.label_text.clone() == "Login:" {self.request_login();}
                else {self.request_signup();}
            }});
    }
}

impl Vapor {
    fn request_login(&mut self){
        // make signup request here
        //let url = format!("https://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net/api/User/LookForUser?username={}", self.user.name);
        self.db_api.get_login(self.user.name.as_str());
    }


    fn request_signup(&mut self){
        //let url = format!("https://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net/api/User/AddUser?username={}&password={}", self.user.name, self.user.password);
        self.db_api.post_signup(self.user.name.as_str(), self.user.password.as_str());
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
