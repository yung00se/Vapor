//use eframe::egui::mutex::Mutex;

use serde_json::Value;
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
    fn get(&self, input: String);
    fn post(&self, url: String); //username: &str, password: &str);
    //fn login(&self, username: &str) -> ReturnType;
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


pub struct DbAPI {
    pub client: Client,
    pub notify: Arc<Notify>,
    pub users: Arc<Mutex<Vec<User>>>,
}


impl DbAPI {

    pub fn new() -> Self{
        Self {
            client: Client::new(),
            notify: Arc::new(Notify::new()),
            users: {Arc::new(Mutex::new(Vec::new()))},
        }
    }
}

impl MakeRequest for DbAPI{
    fn get(&self, url: String) {
        let url = url;
        //let url = format!("http://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net/api/User/LookForUser?username={}", input);
        let response_arc: Arc<Mutex<Vec<User>>> = Arc::clone(&self.users);
        tokio::spawn(async move{
            let response = reqwest::get(url).await;
            match response {
                Ok(resp) => {
                    //let response_body: String = resp.text().await.expect("Failed to get response");
                    //eprint!("{}", response_body);
                    let response_body: Vec<User> = resp.json().await.expect("Error awaiting response");
                    *response_arc.lock().unwrap() = response_body;
                },
                Err(e) => {
                    eprint!("{}", e);
                }
            }
        });
    }

    fn post(&self, url: String) { //username: &str, password: &str) {
        let url = url;
        //let url = format!("https://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net/api/User/AddUser?username={}&password={}", username, password);
        let response_arc: Arc<Mutex<Vec<User>>> = Arc::clone(&self.users);
        // parse payload into JSON here
        //let json_map: User = serde_json::from_str(contents.as_str()).expect("Error");
        let client_clone = self.client.clone();
        tokio::spawn(async move{
            let response = client_clone.post(&url).body("").send().await;
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