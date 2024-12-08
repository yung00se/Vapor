use eframe::egui::{self, epaint};
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
        let dir = env::current_dir().unwrap();
        eprint!("{:?}", dir);
        egui::Window::new("Chat")
            .frame(egui::Frame{
                fill: egui::Color32::from_rgb(92, 30, 38),
                ..Default::default()
            })
            .min_height(200.0)
            .max_height(300.0)
            .collapsible(true)
            .scroll([false, true])//Horizontal Scrolling: False, Vertical Scrolling: True
            .anchor(Align2::LEFT_BOTTOM, [10.0, 0.0])
            .pivot(Align2::LEFT_BOTTOM)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::BOTTOM), |ui|{

                    let input = ui
                        .add(egui::TextEdit::singleline(&mut self.message)
                             .hint_text("Type here..."));

                    let buffer = self.buffer.lock().unwrap();
                    egui::ScrollArea::vertical().show(ui, |ui| { 
                        for message in buffer.iter() { ui.add(egui::Label::new(message)); }
                    });//End Scroll Area

                    if ui.input(|i|{ i.key_pressed(egui::Key::Enter) }) {
                        self.child_stdin.write_all(format!("{}\n", self.message).as_bytes()).expect("Failed to write to chat service");
                        self.message.clear() }
                });//End bottom_up display area
        });//End Window
    }
}
