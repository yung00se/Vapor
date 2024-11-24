use eframe::egui::{Event, FontFamily, FontId, FontSelection, TextEdit, RichText};
use eframe::{App, Frame};
use eframe::egui::{self, CentralPanel, Label, Sense, Color32, Context, text::Fonts, FontDefinitions, Key, Painter, Pos2, Rect, Rounding, Shape, SidePanel, Stroke, TopBottomPanel, Vec2};
use eframe::egui::epaint::{RectShape};
use crate::data_base_api::data_base_api::DbAPI;

pub struct LoginPage {
    pub username: String,
    pub password: String,
    pub error_msg: String,
}

impl Default for LoginPage {
    fn default() -> Self {
        Self {
            username: String::from(""),
            password: String::from(""),
            error_msg: String::from(""),
        }
    }
}

impl App for LoginPage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Username:");
            ui.add(TextEdit::singleline(&mut self.username));
            ui.label("Password:");
            ui.add(TextEdit::singleline(&mut self.password).password(true));
            ui.label(RichText::new(self.error_msg.clone()).color(Color32::RED));

            if ui.input(|i| i.key_pressed(Key::Enter)) {
                self.request_login();
            }
        });
    }
}
impl LoginPage {
    pub fn request_login(&self) -> bool {
        let db_api = DbAPI::new();
        //let result = db_api.login_request(&self.username, &self.password);
        
        true
    }
}