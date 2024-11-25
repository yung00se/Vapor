use crate::data_base_api::DbAPI;
use crate::data_base_api::MakeRequest;
//use crate::pages::login_page::DisplayLanding;
use crate::user_info::User;
use crate::pages::navigator::NavBar;
use eframe::{App, Frame};
use eframe::egui;

pub struct Vapor {
    pub user: User,
    pub db_api: DbAPI,
}

impl Default for Vapor{
    fn default() -> Self{
        Self{
            user: User::default(),
            db_api: DbAPI::new(),
        }
    }
}

impl App for Vapor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        eprint!("now here");
        match self.user.id {
            Some(_id) => {
                self.user.show_nav_bar(ctx);
            }
            None => {
                //self.display_landing(ctx);
                if let Some(user_info) = self.db_api.users.lock().unwrap().pop() {
                    self.user.id = Some(user_info.UserID);
                }
                eprint!("im here");
            }
        }
        // Draw current page
        self.user.show_current_page(ctx);
    }
}

impl Vapor {
    pub fn request_login(&mut self){
        // make signup request here
        self.db_api.get(&self.user.name);
        match self.user.id {
            Some(id) => {
                eprint!("User {} Logged in", id);
            }
            None => {
                eprint!("failed log in");
            }
        }
        self.user.id = Some(420); //Example ID number
        //The implementation should:
        //Check if the user entered a valid username/password combo
        //Assign the ID that corresponds with that account
        self.user.current_page = "lib".to_string();
    }

    pub fn request_signup(&mut self){
        //TODO//
        self.user.id = Some(6969);
        //Similar to request_login, but assign a new unique ID to their account instead
        self.user.id = Some(82317);
    }
}
