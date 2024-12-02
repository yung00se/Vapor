use eframe::egui;
use super::pages::{friends_page::DisplayFriends, leaderboard_page::DisplayLeaderboard, game_hub::{DisplayLibrary, build_library}, navigator::NavBar};
use crate::{data_base_api::{DbAPI, UserEntry}, pages::game_hub::GameIcon};

//#[derive(Clone)]
pub struct User {
    pub name: String,
    pub password: String,
    pub id: i32,
    pub library: Vec<GameIcon>,
    pub friends: Vec<UserEntry>,
    pub leaderboard: Vec<UserEntry>,
    pub current_page: String,
    pub got_response: bool,
}

impl Default for User{
    fn default() -> Self{
        Self {
            name: "".into(),
            password: "".into(),
            id: -1,
            library: Vec::new(),
            friends: Vec::new(),
            leaderboard: Vec::new(),
            current_page: "land".to_string(),
            got_response: false,
        }
    }
}

impl User {
    pub fn new(name: String, password: String, id: i32) -> Self {
        Self {
            name,
            password,
            id,
            library: build_library(),
            friends: Vec::new(),
            leaderboard: Vec::new(),
            current_page: "land".to_string(),
            got_response: false,
        }
    } 
    
    /*
    pub fn populate_friends(&mut self){
        self.friends = Some([UserEntry::new(1,"Paul".into(), "pass123".into(), 420),
                             UserEntry::new(2,"John".into(), "pass123".into(), 111111),
                             UserEntry::new(3, "Will".into(), "pass123".into(), 1234),
                             UserEntry::new(4, "Spencer".into(), "pass123".into(), 909),
                             UserEntry::new(5, "Mann".into(), "pass123".into(), 100000)].to_vec())
    }
    
    pub fn populate_leaderboard(&mut self){
        self.leaderboard = Some([
                UserEntry::new(1, "xx_Slayer_1027789_xx".into(), "pass123".into(), 32),
                UserEntry::new(1, "urmomlol".into(), "pass123".into(), 69),
                UserEntry::new(1, "why_is_this_game_so_hard".into(), "pass123".into(), 0),
                UserEntry::new(1, "vapor_user_1404".into(), "pass123".into(), 1337),
                UserEntry::new(1, "ben_dover".into(), "pass123".into(), 80085),
                UserEntry::new(1, "hawk_tuah_yuh".into(), "pass123".into(), 303),
                UserEntry::new(1, "80085".into(), "pass123".into(), 43110),
                UserEntry::new(1, "hacker_man_17".into(), "pass123".into(), 1010101),
        ].to_vec());
    }
    */

    pub fn show_current_page(&mut self, ctx: &egui::Context){
        if self.current_page == "lib" {self.display_library(ctx)}
        else if self.current_page == "friends" {self.display_friends(ctx)}
        else if self.current_page == "leaderboards" {self.display_leaderboard(ctx)}
    }
}
