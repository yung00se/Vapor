use eframe::egui::FontId;
use eframe::egui::{self, Color32, Sense, Rect, Rounding, Stroke, Vec2};
use eframe::epaint::RectShape;
use std::process::Command;
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
    pub fn show_game_hub(&self, ctx: &egui::Context){
        egui::CentralPanel::default().show(ctx, |ui| {
            for game in self.games.iter() {

                let (game_rect, response) = ui.allocate_exact_size(Vec2::new(200.0, 50.0), Sense::click());
                ui.painter().rect_filled(game_rect, 0.0, Color32::BLACK);
                ui.painter().text(
                    game_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    game.title.clone(),
                    FontId::default(),
                    Color32::WHITE);

                if response.clicked() { game.handle_click() }
            }
        });
    }
}
