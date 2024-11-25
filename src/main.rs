use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use eframe::{App, Frame};
use eframe::egui::{self, Color32, Rect};
use vapor::data_base_api::DbAPI;
use vapor::user_info::User;
use vapor::pages::navigator::NavBar;
                  
#[derive(Default)]
struct Vapor {
    user: User,
    chat_viewport: Arc<AtomicBool>,
    chatbar_friends: Arc<Mutex<Vec<User>>>,
}

impl App for Vapor {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        if self.user.id == None { 
            self.user.current_page = "land".to_string();
            match self.user.id {
                Some(_id) => {
                    self.user.show_nav_bar(ctx);
                }
                None => {
                    self.display_landing(ctx);
                    if let Some(user_info) = self.db_api.users.lock().unwrap().pop() {
                        self.user.id = Some(user_info.UserID);
                    }
                }
            }
        }
        self.user.show_nav_bar(ctx);
        //Draw the current page
        self.user.show_current_page(ctx);


        let friends_list: Arc<Mutex<Vec<User>>> = Arc::clone(&self.chatbar_friends);

        let show_chat_viewport = self.chat_viewport.clone();
        ctx.show_viewport_deferred(
            egui::ViewportId::from_hash_of("chat_viewport"),
            egui::ViewportBuilder::default()
                .with_title("Chat Viewport")
                .with_inner_size([200.0, 100.0]),
            move |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Deferred,
                    "This egui backend doesn't support multiple viewports"
                );
                //Chatbar Start

                egui::CentralPanel::default().show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let friends = friends_list.lock().unwrap();

                                for friend in friends.iter() {
                                    let mut mini_friend_rect = Rect::ZERO;
                                    mini_friend_rect.set_width(ui.available_size().x);
                                    mini_friend_rect.set_height(25.0);
                                    
                                    ui.painter().rect_filled(
                                        mini_friend_rect,
                                        2.0,
                                        Color32::DARK_BLUE);};
                    });
                });          

                //Chatbar End


            },
        );
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();

    let _ = eframe::run_native( // Start Vapor
        "Word Unscrambler", // Set the app title
        native_options, 
        Box::new(|_cc| Ok(Box::new(Vapor::default()))),
    );
}
