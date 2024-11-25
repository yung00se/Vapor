//use eframe::egui::mutex::Mutex;
use std::collections::HashMap;

use std::f32::consts::E;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use reqwest::Client;
use reqwest::Response;
use tokio::runtime::Runtime;
use tokio::{self, sync::Notify};
use core::error;
use std::default;
use reqwest::Error;

pub trait MakeRequest {
    fn get_login(&self, username: &str);
    fn get_friends_list(&self, user_id: &str);
    fn get_leaderboard(&self);
    fn get_user_stats(&self, user_id: &str);
    fn post_signup(&self, username: &str, password: &str);
}

pub enum ReturnType{
    IsValid(bool),
    Users(Vec<User>),
    Error(Option<String>),
    CurrentUser(User),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User{
    pub UserID: i32,
    pub Username: String,
    pub Password: String,
    pub HighScore: i32,
}

impl Default for User {
    fn default() -> Self {
        Self {
            UserID: -1,
            Username: "".to_string(),
            Password: "".to_string(),
            HighScore: 0,
        }
    }
}

pub struct DbAPI {
    pub client: Client,
    pub notify: Arc<Notify>,
    pub user: Arc<Mutex<Vec<User>>>,
    pub friends_list: Arc<Mutex<Vec<String>>>,
    pub leaderboard: Arc<Mutex<Vec<User>>>,
}


impl DbAPI {

    pub fn new() -> Self{
        Self {
            client: Client::new(),
            notify: Arc::new(Notify::new()),
            user: Arc::new(Mutex::new(Vec::new())),
            friends_list: Arc::new(Mutex::new(Vec::new())),
            leaderboard: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl MakeRequest for DbAPI{
    fn get_login(&self, username: &str) {
        let api_url = "https://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net/api".to_string();
        let end = format!("/User/LookForUser?username={}", username);
        let url = api_url + &end;
        eprint!("{}", url);
        let response_arc: Arc<Mutex<Vec<User>>> = Arc::clone(&self.user);
        tokio::spawn(async move{
            let response = reqwest::get(url).await;
            match response {
                Ok(resp) => {
                    let response_body: Vec<User> = resp.json().await.expect("Error Logging in");
                    *response_arc.lock().unwrap() = response_body;
                },
                Err(e) => {
                    eprint!("{}", e);
                }
            }
        });
    }

    fn get_friends_list(&self, user_id: &str) {
        let api_url = "https://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net/api".to_string();
        let end = format!("/Friend/GetAllFriends/{}", user_id);
        let url = api_url + &end;
        eprint!("{}", url);
        // /Friend/GetAllFriends/{UserID}
        let response_arc: Arc<Mutex<Vec<String>>> = Arc::clone(&self.friends_list);
        tokio::spawn(async move{
            let response = reqwest::get(url).await;
            match response {
                Ok(resp) => {
                    let response_body: Vec<String> = resp.json().await.expect("Error getting friends list");
                    *response_arc.lock().unwrap() = response_body;
                },
                Err(e) => {
                    eprint!("{}", e);
                }
            }
        });
    }

    fn get_leaderboard(&self) {
        let url = "https://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net/api/User/GetScoresDescending".to_string();
        eprint!("{}", url);
        let response_arc: Arc<Mutex<Vec<User>>> = Arc::clone(&self.leaderboard);
        tokio::spawn(async move{
            let response = reqwest::get(url).await;
            match response {
                Ok(resp) => {
                    let response_body: Vec<User> = resp.json().await.expect("Error getting leaderboard");
                    *response_arc.lock().unwrap() = response_body;
                },
                Err(e) => {
                    eprint!("{}", e);
                }
            }
        });
    }

    fn get_user_stats(&self, user_id: &str) {

    }

    fn post_signup(&self, username: &str, password: &str) { //username: &str, password: &str) {
        let api_url = "https://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net/api".to_string();
        let end = format!("/User/AddUser?username={}&password={}", username, password);
        let url = api_url + &end;
        // User/AddUser?username=paul&password=firefire"
        let response_arc: Arc<Mutex<Vec<User>>> = Arc::clone(&self.user);
        let client_clone = self.client.clone();
        tokio::spawn(async move{
            let response = client_clone.post(url).body("").send().await;
            match response {
                Ok(resp) => {
                    let response_body: Vec<User> = resp.json().await.unwrap();
                    *response_arc.lock().unwrap() = response_body;
                },
                Err(e) => {
                    if e.status().unwrap() == 400 {
                        eprint!("Username is taken\n");
                    }
                    else {
                        eprint!("{}", e);
                    }
                }
            }
        });
    }
}