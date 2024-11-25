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
    fn get(&self, input: &str);
    fn post(&self, input: &str, payload: &str);
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
    fn get(&self, input: &str) {
        let url = format!("http://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net/api/User/LookForUser?username={}", input);
        let response_arc: Arc<Mutex<Vec<User>>> = Arc::clone(&self.users);
        tokio::spawn(async move{
            let response = reqwest::get(&url).await;
            match response {
                Ok(resp) => {
                    let response_body: Vec<User> = resp.json().await.expect("Error awaiting response");
                    *response_arc.lock().unwrap() = response_body;
                },
                Err(e) => {
                    eprint!("{}", e);
                }
            }
        });
    }

    fn post(&self, input: &str, payload: &str) {
        let url = format!("http://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net/{}", input);
        let response_arc: Arc<Mutex<Vec<User>>> = Arc::clone(&self.users);
        // parse payload into JSON here
        let contents = std::fs::read_to_string("test.json").expect("Failed to read from file");
        let json_map: User = serde_json::from_str(contents.as_str()).expect("Error");
        let client_clone = self.client.clone();
        tokio::spawn(async move{
            let response = client_clone.post(&url).json(&json_map).send().await;
            match response {
                Ok(resp) => {
                    let response_body: Vec<User> = resp.json().await.unwrap();
                    *response_arc.lock().unwrap() = response_body;
                    println!("Sucessful post");
                },
                Err(e) => eprint!("{}", e)}          
        });
    }
}