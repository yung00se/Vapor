//Friend//
#[derive(Clone)]
pub struct Friend {
    pub height: f32,
    pub width: f32,
    pub username: String,
    pub wu_highscore: u16,
}

impl Default for Friend {
    fn default() -> Self {
        Self {
            height: 50.0,
            width: 100.0,
            username: String::from(""),
            wu_highscore: 0,
        }
    }
}

impl Friend {
    fn new(username: String, highscore: u16) -> Self {
        Self {
            height: 50.0,
            width: 100.0,
            username: username,
            wu_highscore: highscore,
        }
    }
}

//Friends Page//
pub struct FriendsPage {
    pub page_name: String,
    pub friends: Vec<Friend>,
}

impl Default for FriendsPage {
    fn default() -> Self {
        Self {
            page_name: String::from("Friends"),
            friends: [ Friend::new(String::from("Paul"), 43),
                       Friend::new(String::from("John"), 26),
                       Friend::new(String::from("Will"), 39),
                       Friend::new(String::from("Spencer"), 38),
                       Friend::new(String::from("Mann"), 46)
                     ].to_vec()
        }
    }
}

