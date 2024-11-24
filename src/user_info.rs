use eframe::egui;
use super::pages::{friends_page::DisplayFriends, login_page::DisplayLanding, leaderboard_page::DisplayLeaderboard, game_hub::DisplayLibrary, navigator::NavBar};
use crate::pages::game_hub::GameIcon;

#[derive(Clone)]
pub struct User {
    pub name: String,
    pub password: String,
    pub id: Option<i32>,
    pub library: Option<Vec<GameIcon>>,
    pub friends: Option<Vec<(User, i32)>>,
    pub leaderboard: Option<Vec<(User, i32)>>,
    pub current_page: String,
}

impl Default for User{
    fn default() -> Self{
        Self {
            name: "".into(),
            password: "".into(),
            id: None,
            library: None,
            friends: None,
            leaderboard: None,
            current_page: "land".to_string(),
        }
    }
}



impl User {
    pub fn new(name: String) -> Self {
        Self {
            name,
            password: "".into(),
            id: Some(000_i32),
            library: None,
            friends: None,
            leaderboard: None,
            current_page: "land".to_string(),
        }
    } 
    
    pub fn populate_library(&mut self){
        
    }
    
    pub fn populate_friends(&mut self){
        self.friends = Some([(User::new("Paul".into()), 420),
                             (User::new("John".into()), 111111),
                             (User::new("Will".into()), 1234),
                             (User::new("Spencer".into()), 909),
                             (User::new("Mann".into()), 100000)].to_vec())
    }
    
    pub fn populate_leaderboard(&mut self){
        self.leaderboard = Some([(User::new(String::from("xx_Slayer_1027789_xx")), 32),
                                (User::new(String::from("urmomlol")), 69),
                                (User::new(String::from("why_is_this_game_so_hard")), 0),
                                (User::new(String::from("vapor_user_1404")), 1337),
                                (User::new(String::from("ben_dover")), 80085),
                                (User::new(String::from("hawk_tuah_yuh")), 303),
                                (User::new(String::from("80085")), 43110),
                                (User::new(String::from("hacker_man_17")), 1010101)].to_vec());
    }

    pub fn show_current_page(&mut self, ctx: &egui::Context){
        if self.current_page == "lib" {self.display_library(ctx)}
        else if self.current_page == "friends" {self.display_friends(ctx)}
        else if self.current_page == "leaderboards" {self.display_leaderboard(ctx)}
        else {self.display_landing(ctx)}
    }
}
