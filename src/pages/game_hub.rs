use eframe::{egui::{self, Color32, FontId, Pos2, Rect, Rounding, Sense, Shape, Stroke, Vec2},
             epaint::RectShape};
use walkdir::WalkDir;
use std::{env, path::PathBuf, process::{Command, Stdio}};
use std::io::{Read, Write, BufRead, BufReader};
use std::fs;

//use crate::user_info::User;
use crate::vapor::Vapor;

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
    pub fn run_game(&self) {
       let mut game_instance = 
            Command::new(&self.path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Game not found in Vapor Path...");

        if let Some(game_output) = &mut game_instance.stdout {
            let lines = BufReader::new(game_output).lines().enumerate().take(64);
            for line in lines {
                println!("Word Scrambler: {:?}", line);
            }
        }
    }

    fn generate_icon_rect(&mut self){
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

pub trait DisplayLibrary{
    fn display_library(&mut self, ctx: &egui::Context);
    fn get_library_path(&mut self);
}

impl DisplayLibrary for Vapor {
    fn get_library_path(&mut self){
        
    }

    fn display_library(&mut self, ctx: &egui::Context){
        let game_library = build_library();
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
    }
}

pub fn build_library() -> Vec<GameIcon>{   
    let mut games: Vec<GameIcon> = Vec::new();
    
    let mut path = PathBuf::new();

    match env::current_exe(){
        Ok(vapor_path) => { 
            path.push(vapor_path);
            path.pop();
            path.push("library");
            eprint!("Path to Game Library: {:?}", &path);
        },
        Err(e) => eprint!("Error fetching path to Vapor exe: {e}"),
    };

    if !fs::exists(&path).expect("Error evaluating path to game library..."){
        fs::create_dir(&path).expect("Could not create game library directory");
    }
        for result in WalkDir::new(&path){
            let entry = result.expect("No File...");
            let filename = entry.file_name().to_str().expect("Error converting game file-name from osStr => &str");
            let path = entry.path().to_str().expect("Error unwrapping path");
            
            let mut icon = GameIcon{
                title: filename.into(),
                id: games.len() as i16,
                rect: Shape::Noop,
                path: path.into()};
            icon.generate_icon_rect();
            eprint!("{:?}", icon.id);
            games.push(icon);
            eprint!("Icon for: {} created...", filename)
        };

    games
}


