use eframe::{App, Frame};
use eframe::egui;
use vapor::user_info::User;
use vapor::pages::navigator::NavBar;
                  

struct Vapor {
    user: User,
}

impl Default for Vapor{
    fn default() -> Self{
        Self{
            user: User::default(),
        }
    }
}

impl App for Vapor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
            if self.user.id == None { self.user.current_page = "land".to_string();}

        self.user.show_nav_bar(ctx);
        //Draw the current page
        self.user.show_current_page(ctx);
    }
}


#[tokio::main]
async fn main() {
    let native_options = eframe::NativeOptions::default();

    let _ = eframe::run_native( // Start Vapor
        "Word Unscrambler", // Set the app title
        native_options, 
        Box::new(|_cc| Ok(Box::new(Vapor::default()))),
    );
}
