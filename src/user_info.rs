use super::pages::game_hub::build_library;
use crate::{data_base_api::UserEntry, pages::game_hub::GameIcon};

pub struct User {
    pub name: String,
    pub password: String,
    pub id: i32,
    pub library: Vec<GameIcon>,
    pub friends: Vec<UserEntry>,
    pub leaderboard: Vec<UserEntry>,
    pub current_page: String,
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
        }
    } 
}
