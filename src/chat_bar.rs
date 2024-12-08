use eframe::egui;
use emath::Align2;
use crate::vapor::Vapor;
use std::{fs::{self, File}, io::{BufRead, BufReader, Write}, process::{Child, ChildStdin, ChildStdout, Command, Stdio}, sync::{Arc, Mutex}, time::Duration};

pub struct Chat{
    buffer: Arc<Mutex<Vec<String>>>,
    message: String,
    child_stdin: ChildStdin,
}

impl Chat{
    pub fn new() -> Self{
        let mut child = Command::new("python")
            .arg("-u")
            .arg("./python_client/main.py")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start python script");

        let buffer: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let buffer_clone = Arc::clone(&buffer);
        //let child_stdout = child.stdout.expect("Failed to get stdout of child");
        

        std::thread::spawn(move || {
            if let Some(chat_output) = &mut child.stdout {
                let reader = BufReader::new(chat_output).lines().enumerate().take(128);

                for (size, line) in reader{
                    let output = line.expect("Failed");
                    buffer_clone.lock().unwrap().push(output);
                }
            }
            else {}
        });

        Self{
            buffer,
            message: String::new(),
            child_stdin: child.stdin.take().expect("Failed to get stdin of child"),
        }
    }
}

pub trait ChatBar{
    fn display_chat_bar(&mut self, ctx: &egui::Context);
}

impl ChatBar for Chat {
    fn display_chat_bar(&mut self, ctx: &egui::Context) {
        egui::Window::new("Chat")
            .min_height(200.0)
            .max_height(300.0)
            .collapsible(true)
            .scroll([false, true])//Horizontal Scrolling: False, Vertical Scrolling: True
            .anchor(Align2::LEFT_BOTTOM, [10.0, 0.0])
            .pivot(Align2::LEFT_BOTTOM)
            .show(ctx, |ui| {
            let buffer = self.buffer.lock().unwrap();
            for message in buffer.iter() {
                ui.add(egui::Label::new(message));
            }

            let input = ui.add(
                egui::TextEdit::singleline(&mut self.message)
                    .hint_text("Type here..."));
            if ui.input(|i|{ i.key_pressed(egui::Key::Enter) }) {
                self.child_stdin.write_all(format!("{}\n", self.message).as_bytes()).expect("Failed to write to chat service");
                self.message.clear();
            }
        });
    }
}
