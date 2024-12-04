use eframe::{egui::IconData, glow::NativeVertexArray, NativeOptions};
use vapor::vapor::Vapor;
//use eframe::IconData;
use image::ImageReader;


#[tokio::main]
async fn main() {
    let native_options = NativeOptions::default();

    let _ = eframe::run_native( // Start Vapor
        "Word Unscrambler", // Set the app title
        native_options, 
        Box::new(|_cc| Ok(Box::new(Vapor::default()))),
    );
}
