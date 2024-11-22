use eframe::egui::{Event, FontFamily, FontId, FontSelection, TextEdit};
use eframe::{App, Frame};
use eframe::egui::{self, CentralPanel, Label, Color32, Context, Sense, text::Fonts, FontDefinitions, Key, Painter, Pos2, Rect, Rounding, Shape, SidePanel, Stroke, TopBottomPanel, Vec2};
use eframe::epaint::{RectShape, TextureId, WHITE_UV};
use std::process::{Command, Output};
use std::env;

pub struct GameIcon {
    pub height: f32,
    pub width: f32,
    pub title: String,
    pub id: i32,
    pub rect: RectShape,
}

impl Default for GameIcon {
    fn default() -> Self {
        let stroke = Stroke::new(1.0, Color32::BLACK);
        Self {
            height: 100.0,
            width: 100.0,
            title: String::from(""),
            id: -1,
            rect: RectShape::new(Rect::ZERO, Rounding::ZERO, Color32::BLACK, stroke),
        }
    }
}

pub struct GameHub {
    pub page_name: String,
    pub games: Vec<GameIcon>,
}


impl Default for GameHub {
    fn default() -> Self {
        // initialize game icons
        let mut game_icons = Vec::new();
        let mut ws = GameIcon::default();
        ws.title = String::from("Word Scramble");
        ws.id = 1;
        let mut other = GameIcon::default();
        other.title = String::from("Other");
        other.id = 2;
        game_icons.push(ws);
        game_icons.push(other);
        Self {
            page_name: String::from("GameHub"),
            games: game_icons,
        }
    }
}

impl GameIcon {
    pub fn handle_click(&self) {
        match self.id {
            1 => {
                println!("Clicked on Word Unscramble Game");
                match env::current_dir() {
                    Ok(path) => {
                        println!("{}", path.display());
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
                let path = ".../word-scramble/target/debug/word_scrambler.exe";
                Command::new(path)
                    .output()
                    .expect("Failed to execute");
            }
            2 => {
                println!("Clicked on Other Game");
            }
            _ => ()
        }
    }
}

impl GameHub {

}
/* 
impl App for GameHub {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if !self.logged_in {
            CentralPanel::default().show(ctx, |ui| {
                ui.label("Username:");
                ui.add(TextEdit::singleline(&mut self.username));
                ui.label("Password:");
                ui.add(TextEdit::singleline(&mut self.password).password(true));
    
                if ui.input(|i| i.key_pressed(Key::Enter)) {
                    self.submit_auth();
                }
            });
        }

        else {
            CentralPanel::default().show(ctx, |ui| {
                /*
                ui.horizontal(|ui| {
                    for page in &self.pages {
                        if ui
                            .add(Label::new(page.title.clone()).sense(Sense::click()))
                            .clicked()
                            {
                                page.handle_click();
                                ui.add_space(75.0);
                            }
                    }
                });
                */
                ui.horizontal(|ui| {
                    if ui
                        .add(Label::new("Games").sense(Sense::click()))
                        .clicked()
                        {
                            self.handle_click(1);
                        }
                    ui.add_space(75.0);
                    if ui
                        .add(Label::new("Friends").sense(Sense::click()))
                        .clicked()
                        {
                            self.handle_click(2);
                        }
                    ui.add_space(75.0);
                    if ui
                        .add(Label::new("Leaderboards").sense(Sense::click()))
                        .clicked()
                        {
                            self.handle_click(3);
                        }
                });
                ui.add_space(75.0);
                // create game icons
                let (word_scramble_icon, ws_response) = ui.allocate_exact_size(Vec2::new(200.0, 50.0), Sense::click());
                let (other_icon, other_response) = ui.allocate_exact_size(Vec2::new(200.0, 50.0), Sense::click());

                // initialize icon border colors
                let mut ws_stroke = Stroke::new(2.0, Color32::BLACK);
                let mut other_stroke = Stroke::new(2.0, Color32::BLACK);

                // check for hover on all icons
                if ws_response.hovered() {
                    ws_stroke.color = Color32::LIGHT_BLUE;
                }
                if other_response.hovered() {
                    other_stroke.color = Color32::LIGHT_BLUE;
                }

                // check for click on all icons
                if ws_response.clicked() {
                    self.handle_click(4);
                }
                if other_response.clicked() {
                    self.handle_click(5);
                }
                // paint the icons
                ui.horizontal(|ui| {
                    ui.painter().rect_stroke(word_scramble_icon, 0.0, ws_stroke);
                    // add text
                    ui.painter().text(
                        word_scramble_icon.center(),
                        egui::Align2::CENTER_CENTER,
                        "Word Unscramble",
                        FontId::default(),
                        Color32::BLACK,
                    );
                    ui.add_space(50.0);
                    ui.painter().rect_stroke(other_icon, 0.0, other_stroke);
                    // add text
                    ui.painter().text(
                        other_icon.center(),
                        egui::Align2::CENTER_CENTER,
                        "Other Game",
                        FontId::default(),
                        Color32::BLACK,
                    );
                });
            });
        }
    }
}

impl GameHub {
    pub fn add_game(&mut self, height: f32, width: f32, title: String, id: i32) -> &mut Self{
        let mut game = GameIcon::default();
        game.height = height;
        game.width = width;
        game.title = title;
        game.id = id;
        self.games.push(game);
        self
    }

    // add code for sending login credentials to API here
    fn submit_auth(&mut self) {
        // make API call for login here
        println!("Attempting login");
        self.logged_in = true;
    }

    // handle clicking on the various icons on the screen
    fn handle_click(&mut self, target: u8) -> bool {
        match target {
            1 => {
                println!("Clicked on Games Page");
                false
            }
            2 => {
                println!("Clicked on Friends Page");
                false
            }
            3 => {
                println!("Clicked on Leaderboard Page");
                false
            }
            4 => {
                println!("Clicked on Word Unscramble Game");
                let path = "../Word_Unscrambler/target/debug/world_scrambler.exe";
                Command::new(path)
                    .output()
                    .expect("Failed to execute");
                true
            }
            5 => {
                println!("Clicked on Other Game");
                true
            }
            _ => false
        }
    }

    fn get_friends_list() {
        // Make API request to get a list of friends and their scores
    }

    fn get_leaderboards() {
        // Make API request for the top of the leaderboard
    }
}
    */