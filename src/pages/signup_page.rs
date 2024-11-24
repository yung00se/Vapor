use eframe::egui::{Event, FontFamily, FontId, FontSelection, TextEdit};
use eframe::{App, Frame};
use eframe::egui::{self, CentralPanel, Label, Sense, Color32, Context, text::Fonts, FontDefinitions, Key, Painter, Pos2, Rect, Rounding, Shape, SidePanel, Stroke, TopBottomPanel, Vec2};
use eframe::egui::epaint::{RectShape};

pub struct SignupPage {
    pub username: String,
    pub password: String,
    pub error_msg: String,
}

impl Default for SignupPage {
    fn default() -> Self {
        Self {
            username: String::from(""),
            password: String::from(""),
            error_msg: String::from(""),
        }
    }
}

impl App for SignupPage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Username:");
            ui.add(TextEdit::singleline(&mut self.username));
            ui.label("Password:");
            ui.add(TextEdit::singleline(&mut self.password).password(true));

            if ui.input(|i| i.key_pressed(Key::Enter)) {
                self.request_signup();
            }
        });
    }
}
impl SignupPage {
    pub fn request_signup(&self) -> bool{
        // make signup request here
        //println!("Username: {}\nPassword: {}", self.username, self.password);
        false
    }
}