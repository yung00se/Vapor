use eframe::egui;
use crate::vapor::Vapor;

pub struct Chat{
    buffer: Vec<String>,
    message: String,
}

impl Chat{
    pub fn new() -> Self{
        Self{
            buffer: Vec::new(),
            message: String::new(),
        }
    }
}

pub trait ChatBar{
    fn display_chat_bar(&mut self, ctx: &egui::Context);
}

impl ChatBar for Chat {
    fn display_chat_bar(&mut self, ctx: &egui::Context){
        egui::Window::new("Chat").show(ctx, |ui| {
            for message in &self.buffer{
                ui.add(egui::Label::new(message));
            }

            let input = ui.add(
                egui::TextEdit::singleline(&mut self.message)
                    .hint_text("Type here..."));
            if ui.input(|i|{ i.key_pressed(egui::Key::Enter) }) {
                self.buffer.push(self.message.clone());
                self.message.clear();
            }
        });
    }
}
