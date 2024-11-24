use eframe::egui::{Label, RichText, Sense, TextEdit, TopBottomPanel};
use eframe::egui::{self, CentralPanel, Color32, Key};
use crate::data_base_api::data_base_api::DbAPI;
use crate::user_info::User;

pub trait DisplayLanding{
    fn display_landing(&mut self, ctx: &egui::Context);
}

impl DisplayLanding for User{
    fn display_landing(&mut self, ctx: &egui::Context){
        let mut label_text = String::new();
        TopBottomPanel::bottom("login-or-signup").show(ctx, |ui| {
            ui.horizontal(|ui| { if ui
                                 .add(Label::new("Login").sense(Sense::click()))
                                 .clicked(){ label_text = "Login:".into() }

                                 ui.add_space(75.0);

                                 if ui
                                 .add(Label::new("Signup").sense(Sense::click()))
                                 .clicked() { label_text = "Sign Up:".into()  } /*End Login/Signup Buttons*/ });
        });
        CentralPanel::default().show(ctx, |ui| {
            ui.heading(label_text.clone());
            
            ui.label("Username:");
            ui.add(TextEdit::singleline(&mut self.name)); //Username Entry

            ui.label("Password:");
            ui.add(TextEdit::singleline(&mut self.password).password(true));

            ui.label(RichText::new("Test").color(Color32::RED));

            if ui.input(|i| i.key_pressed(Key::Enter)) { 
                if label_text.clone() == "Login:" {self.request_login();}
                else {self.request_signup();}
            }});
    }
}


impl User {
    fn request_login(&mut self){
        // make signup request here
        let db_api = DbAPI::new();
        let user_logged_in = db_api.login_request(&self.name, &self.password);
        println!("{}", user_logged_in);
        if (user_logged_in) {
            println!("Logged in");
        }
        self.id = Some(420); //Example ID number
        //The implementation should:
        //Check if the user entered a valid username/password combo
        //Assign the ID that corresponds with that account
        self.current_page = "lib".to_string();
    }

    fn request_signup(&mut self){
        //TODO//
        self.id = Some(6969);
        //Similar to request_login, but assign a new unique ID to their account instead
        self.id = Some(82317);
    }
}
