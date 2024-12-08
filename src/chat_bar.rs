use eframe::egui::{self, epaint, Color32};
use emath::Align2;
use std::{env, io::{BufRead, BufReader, Write}, process::{Child, ChildStdin, ChildStdout, Command, Stdio}, sync::{Arc, Mutex}, time::Duration};

pub struct Chat{
    buffer: Arc<Mutex<Vec<String>>>,
    message: String,
    child_stdin: ChildStdin,
}

impl Chat{
    pub fn new() -> Self{
        let mut child = Command::new("python3")
            .arg("-u")
            .arg("./python_client/main.py")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start python script");

        let buffer: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let buffer_clone = Arc::clone(&buffer);

        std::thread::spawn(move || {
            if let Some(chat_output) = &mut child.stdout {
                let reader = BufReader::new(chat_output).lines().enumerate().take(128);

                for (_, line) in reader{
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
            .frame(egui::Frame{
                fill: egui::Color32::from_rgb(92, 30, 38),
                rounding: egui::Rounding::same(5.0),
                ..Default::default()
            })
            .min_height(200.0)
            .max_height(300.0)
            .collapsible(true)
            .scroll([false, true])//Horizontal Scrolling: False, Vertical Scrolling: True
            .anchor(Align2::LEFT_BOTTOM, [10.0, 0.0])
            .pivot(Align2::LEFT_BOTTOM)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui|{

                    egui::Frame::none()//Blank frame for styling the text edit box
                        .fill(egui::Color32::from_rgb(56, 18, 23))
                        .show(ui, |ui|{
                            ui.add(egui::TextEdit::singleline(&mut self.message)
                                   .return_key(None)
                                   .frame(false)//Override default text edit style
                                   .text_color(egui::Color32::from_rgb(252, 251, 182))
                                   .desired_width(ui.available_width())
                                   .hint_text("Type here..."));
                        });//End text input

                    let buffer = self.buffer.lock().unwrap();
                    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                        egui::ScrollArea::vertical()
                            .stick_to_bottom(true)
                            .show(ui, |ui| { 
                                ui.set_width(ui.available_width());
                                for message in buffer.iter() { ui.colored_label(
                                    egui::Color32::from_rgb(252, 251, 182), 
                                    message); }
                            });//End Scroll Area
                    });

                    if ui.input(|i|{ i.key_pressed(egui::Key::Enter) }) {
                        self.child_stdin
                            .write_all(format!("{}\n", self.message).as_bytes())
                            .expect("Failed to write to chat service");
                        self.message.clear() }
                });//End bottom_up display area
            });//End Window
    }
}
