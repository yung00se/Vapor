use eframe::{egui::{self, Color32, FontId, Pos2, Rect, Rounding, Sense, Shape, Stroke, Vec2},
             epaint::RectShape};

use std::process::Command;

use crate::user_info::User;

#[derive(Clone)]
pub struct GameIcon {
    pub title: String,
    pub id: i16,
    pub rect: Shape,
    pub path: String,
}
impl Default for GameIcon {
    fn default() -> Self {
        Self { title: String::from(""),
               id: 0,
               rect: Shape::Noop,
               path: "".into()}
    }
}

impl GameIcon{
    fn generate_icon(&mut self){
        let index = self.id as f32;
        let top_left = Pos2::from(     ((100.0 * index) +  5.0   ,  5.0));
        let bottom_right = Pos2::from( ((100.0 * index) +  105.0 , 55.0));

        self.rect = Shape::from(RectShape::new(
            Rect::from([top_left ,bottom_right]), 
            Rounding::ZERO, 
            Color32::BLACK, 
        Stroke::new(
            1.0,
            Color32::BLACK)))
    }
}

impl GameIcon {
    pub fn run_game(&self) {
        Command::new(&self.path)
            .spawn()
            .expect("Game not found in Vapor Path...");
    }
}

pub trait DisplayLibrary{
    fn display_library(&mut self, ctx: &egui::Context);
}

impl DisplayLibrary for User{
    fn display_library(&mut self, ctx: &egui::Context){
        if let Some(game_library) = &self.library {
            egui::CentralPanel::default().show(ctx, |ui| {
                for game in game_library.iter() {
                    let (game_rect, response) = ui.allocate_exact_size(Vec2::new(200.0, 50.0), Sense::click());
                    ui.painter().rect_filled(game_rect, 0.0, Color32::BLACK);
                    ui.painter().text(
                        game_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        game.title.clone(),
                        FontId::default(),
                        Color32::WHITE,
                    );
                    if response.clicked() { game.run_game() }}
            });
        } else { &mut self.populate_library(); }
    }

    
}

