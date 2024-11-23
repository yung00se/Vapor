
pub mod data_base_api{
    use std::f32::consts::E;
    use std::sync::{Arc, Mutex};
    use serde::Deserialize;
    use reqwest::Client;
    use reqwest::Response;
    use tokio::runtime::Runtime;
    use tokio::{self, sync::Notify};
    use core::error;
    use std::default;
    pub trait MakeRequest {
        fn send_request(&self, input: &str) -> ReturnType;
        fn login(&self, username: &str) -> Option<User>;
    }

    pub enum ReturnType{
        IsValid(bool),
        Users(Vec<User>),
        Error(Option<String>),
        CurrentUser(User),
    }

    #[derive(Deserialize, Debug)]
    pub struct User{
        pub UserID: i32,
        pub Username: String,
        pub Password: String,
        pub HighScore: i32,
    }


    pub struct DbAPI {
        pub client: Client,
        pub notify: Arc<Notify>,
        pub friends: Arc<Mutex<Vec<User>>>,
        pub users: Arc<Mutex<Vec<User>>>,
    }


    impl DbAPI {

        pub fn new() -> Self{
            let api = DbAPI {
                client: Client::new(),
                notify: Arc::new(Notify::new()),
                friends: Arc::new(Mutex::new(Vec::new())),
                users: {Arc::new(Mutex::new(Vec::new()))},
            };
            api.send_request("/api/User/GetAllUsers");

            api
        }
        
        pub fn login_request(&self, username: &str, password: &str) -> bool {
            let user = self.login(username);
            match user {
                Some(u) => {
                    if u.Password != password {
                        false
                    }
                    else {
                        true
                    }
                },
                None => false,
            }
    }
}

    impl MakeRequest for DbAPI{
        fn send_request(&self, input: &str) -> ReturnType{
            let url = format!("http://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net{}", input);
            let response_arc: Arc<Mutex<Vec<User>>> = Arc::clone(&self.users);

            tokio::spawn(async move{
                let response = reqwest::get(&url).await;
                match response{
                    Ok(resp) => {
                        let response_body: Vec<User> = resp.json().await.unwrap();
                        *response_arc.lock().unwrap() = response_body;
                    },
                    Err(e) => eprint!("{}", e)}          
            });
            ReturnType::Error(None)
        }

        fn login (&self, username: &str) -> Option<User> {
            let url = format!("http://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net/api/User/LookForUser?username={}", username);
            let user: Result<User, reqwest::Error>;
            tokio::spawn(async move {
                let response = reqwest::get(&url).await;
                match response {
                    Ok(resp) => {
                        let user: User = resp.json().await.unwrap();
                        Ok(user)
                    },
                    Err(e) => Err(e),
                }
            });
            None

        }
    }
}
