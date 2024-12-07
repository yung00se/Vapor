use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use reqwest::Client;
use tokio;

pub trait MakeRequest {
    fn get_login(&self, username: &str);
    fn get_user_list(&self);
    fn get_friends_list(&self, user_id: &str);
    fn get_leaderboard(&self);
    fn get_user_stats(&self, user_id: &str);
    fn post_signup(&self, username: &str, password: &str);
    fn add_friend(&self, username: &str, friend: &str);
}

/* 
pub enum ReturnType{
    IsValid(bool),
    Users(Vec<User>),
    Error(Option<String>),
    CurrentUser(User),
}
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct UserEntry {
    pub UserID: i32,
    pub Username: String,
    pub Password: String,
    pub HighScore: i32,
}

impl Default for UserEntry {
    fn default() -> Self {
        Self {
            UserID: -1,
            Username: "".to_string(),
            Password: "".to_string(),
            HighScore: 0,
        }
    }
}

impl UserEntry {
    pub fn new(id: i32, name: String, pass: String, score: i32) -> Self {
        Self {
            UserID: id,
            Username: name,
            Password: pass,
            HighScore: score,
        }
    }
}

pub struct DbAPI {
    pub client: Client,
    pub user: Arc<Mutex<Vec<UserEntry>>>,
    pub friends_list: Arc<Mutex<Vec<String>>>,
    pub user_list: Arc<Mutex<Vec<UserEntry>>>,
    pub leaderboard: Arc<Mutex<Vec<UserEntry>>>,
}

impl DbAPI {
    pub fn new() -> Self{
        Self {
            client: Client::new(),
            user: Arc::new(Mutex::new(Vec::new())),
            friends_list: Arc::new(Mutex::new(Vec::new())),
            user_list: Arc::new(Mutex::new(Vec::new())),
            leaderboard: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl MakeRequest for DbAPI{
    fn get_login(&self, username: &str) {
        let api_url = "https://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net/api".to_string();
        let end = format!("/User/LookForUser?username={}", username);
        let url = api_url + &end;
        //eprint!("{}", url);
        let user_arc: Arc<Mutex<Vec<UserEntry>>> = Arc::clone(&self.user);
        tokio::spawn(async move{
            let response = reqwest::get(url).await;
            match response {
                Ok(resp) => {
                    let response_body: Vec<UserEntry> = resp.json().await.expect("Error Logging in");
                    *user_arc.lock().unwrap() = response_body;
                },
                Err(e) => {
                    eprint!("{}", e);
                }
            }
        });
    }

    fn get_user_list(&self) {
        let api_url = "https://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net/api".to_string();
        let end = format!("/User/GetAllUsers");
        let url = api_url + &end;
        eprint!("{}", url);
        // /Friend/GetAllFriends/{UserID}
        let user_list_arc: Arc<Mutex<Vec<UserEntry>>> = Arc::clone(&self.user_list);
        tokio::spawn(async move{
            let response = reqwest::get(url).await;
            match response {
                Ok(resp) => {
                    let response_body: Vec<UserEntry> = resp.json().await.expect("Error getting friends list");
                    *user_list_arc.lock().unwrap() = response_body;
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
        // /Friend/GetAllFriends/{UserID}
        let friends_list_arc: Arc<Mutex<Vec<String>>> = Arc::clone(&self.friends_list);
        tokio::spawn(async move{
            let response = reqwest::get(url).await;
            match response {
                Ok(resp) => {
                    let response_body: Vec<String> = resp.json().await.expect("Error getting friends list");
                    *friends_list_arc.lock().unwrap() = response_body;
                },
                Err(e) => {
                    eprint!("{}", e);
                }
            }
        });
    }

    fn get_leaderboard(&self) {
        let url = "https://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net/api/User/GetScoresDescending".to_string();
        let leaderboard_arc: Arc<Mutex<Vec<UserEntry>>> = Arc::clone(&self.leaderboard);
        tokio::spawn(async move{
            let response = reqwest::get(url).await;
            match response {
                Ok(resp) => {
                    let response_body: Vec<UserEntry> = resp.json().await.expect("Error getting leaderboard");
                    //let response_body: String = resp.text().await.expect("Error getting leaderboard");
                    //eprint!("{}\n", response_body);
                    *leaderboard_arc.lock().unwrap() = response_body;
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
        let response_arc: Arc<Mutex<Vec<UserEntry>>> = Arc::clone(&self.user);
        let client_clone = self.client.clone();
        tokio::spawn(async move{
            let response = client_clone.post(url).body("").send().await;
            match response {
                Ok(resp) => {
                    let response_body: Vec<UserEntry> = resp.json().await.unwrap();
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

    fn add_friend(&self, username: &str, friend: &str) {
        let api_url = "https://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net/api".to_string();
         // this needs to change: let end = format!("/User/AddUser?username={}&password={}", username, password);
        let url = api_url;// + &end;
        // User/AddUser?username=paul&password=firefire"
        let response_arc: Arc<Mutex<Vec<UserEntry>>> = Arc::clone(&self.user);
        let client_clone = self.client.clone();
        tokio::spawn(async move{
            let response = client_clone.post(url).body("").send().await;
            match response {
                Ok(resp) => {
                    match resp.json().await{
                        Ok(list) => {*response_arc.lock().unwrap() = list},
                        Err(e) => eprint!("{e}"),
                    }
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
