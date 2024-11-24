use eframe::egui::{Event, FontFamily, FontId, FontSelection, TextEdit};
use eframe::{App, Frame};
use eframe::egui::{self, CentralPanel, Label, Color32, Context, Sense, text::Fonts, FontDefinitions, Key, Painter, Pos2, Rect, Rounding, Shape, SidePanel, Stroke, TopBottomPanel, Vec2};
use eframe::epaint::{RectShape};


pub struct User {
    pub height: f32,
    pub width: f32,
    pub username: String,
    pub wu_highscore: u16,
}

impl Default for User {
    fn default() -> Self {
        Self {
            height: 50.0,
            width: 100.0,
            username: String::from(""),
            wu_highscore: 0,
        }
    }
}

impl User {
    fn new(username: String, wu_highscore: u16) -> Self {
        Self {
            height: 50.0,
            width: 100.0,
            username: username,
            wu_highscore: wu_highscore,
        }
    } 
}

pub struct LeaderboardPage {
    pub page_name: String,
    pub users: Vec<User>,
}


impl Default for LeaderboardPage {
    fn default() -> Self {
        let mut users = Vec::new();
        users.push(User::new(String::from("xx_Slayer_1027789_xx"), 32));
        users.push(User::new(String::from("urmomlol"), 112));
        users.push(User::new(String::from("why_is_this_game_so_hard"), 0));
        users.push(User::new(String::from("vapor_user_1404"), 49));
        users.push(User::new(String::from("ben_dover"), 75));
        users.push(User::new(String::from("hawk_tuah_yuh"), 69));
        users.push(User::new(String::from("80085"), 88));
        users.push(User::new(String::from("hacker_man_17"), 65535));
        Self {
            page_name: String::from("Leaderboard"),
            users: users,
        }
    }
}

impl LeaderboardPage {

}