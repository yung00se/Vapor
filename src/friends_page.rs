use eframe::egui::{Event, FontFamily, FontId, FontSelection, TextEdit};
use eframe::{App, Frame};
use eframe::egui::{self, CentralPanel, Label, Color32, Context, Sense, text::Fonts, FontDefinitions, Key, Painter, Pos2, Rect, Rounding, Shape, SidePanel, Stroke, TopBottomPanel, Vec2};
use eframe::epaint::{RectShape};
use std::process::{Command, Output};


pub struct Friend {
    pub height: f32,
    pub width: f32,
    pub username: String,
    pub highscore: u16,
}

impl Default for Friend {
    fn default() -> Self {
        Self {
            height: 50.0,
            width: 100.0,
            username: String::from(""),
            highscore: 0,
        }
    }
}

impl Friend {
    fn new(username: String, highscore: u16) -> Self {
        Self {
            height: 50.0,
            width: 100.0,
            username: username,
            highscore: highscore,
        }
    }
}

pub struct FriendsPage {
    pub page_name: String,
    pub friends: Vec<Friend>,
}


impl Default for FriendsPage {
    fn default() -> Self {
        let mut friends = Vec::new();
        friends.push(Friend::new(String::from("Paul"), 43));
        friends.push(Friend::new(String::from("John"), 26));
        friends.push(Friend::new(String::from("Will"), 39));
        friends.push(Friend::new(String::from("Spencer"), 38));
        friends.push(Friend::new(String::from("Mann"), 46));
        Self {
            page_name: String::from("Friends"),
            friends: friends,
        }
    }
}

impl FriendsPage {

}