use crossterm::event::read;
use eframe::egui::{self, epaint, Button, Color32};
use emath::Align2;
use reqwest::header::SERVER;
use tokio::stream;
use std::{env, io::{BufRead, BufReader, Write}, process::{Child, ChildStdin, ChildStdout, Command, Stdio}, sync::{Arc, Mutex}, time::Duration};
use std::net::TcpStream;
use std::io::prelude::Read;

const SERVER_ADDR: &str = "127.0.0.1:8080";

pub struct Chat{
    pub username: String,
    chat_input: String,
    read_buffer: Arc<Mutex<Vec<String>>>,
    message_list: Arc<Mutex<Vec<String>>>,
    write_stream: TcpStream,
}

impl Chat{
    pub fn new() -> Self{
        let write_stream = TcpStream::connect(SERVER_ADDR).expect("Failed to connect to server");

        let read_stream = Arc::new(Mutex::new(TcpStream::connect(SERVER_ADDR).expect("Failed to connect to server")));
        let read_stream_clone = Arc::clone(&read_stream);

        let message_list = Arc::new(Mutex::new(Vec::new()));
        let message_list_clone = Arc::clone(&message_list);
        
        // reader thread
        std::thread::spawn(move || {
            let mut stream = read_stream_clone.lock().unwrap();
            let reader = BufReader::new(&mut *stream);

            for line_result in reader.lines() {
                let line = line_result.expect("Failed to read from stream");

                message_list_clone
                    .lock()
                    .unwrap()
                    .push(String::from(line));
            }});

        Self{
            username: "".into(),
            chat_input: "".into(),
            read_buffer: Arc::new(Mutex::new(Vec::new())),
            message_list,
            write_stream,
        }
    }
}

pub trait ChatBar{
    //fn send_message_getter(&self) -> String;
    //fn message_list_getter(&self) -> Vec<String>;
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
                            ui.add(egui::TextEdit::singleline(&mut self.chat_input)
                                   .return_key(None)
                                   .frame(false)//Override default text edit style
                                   .text_color(egui::Color32::from_rgb(252, 251, 182))
                                   .desired_width(ui.available_width())
                                   .hint_text("Type here..."));

                            let send_button = ui.add(Button::new("Send"));
                            if send_button.clicked(){
                                self.write_stream.write_all(format!("{}\n",self.chat_input.clone()).as_bytes()).expect("Failed to send message to server");
                                self.chat_input = String::new();
                            }
                            
                            let messages_result = self.message_list.lock().unwrap();
                            for message in messages_result.iter(){
                                ui.label(message);
                            }
                        });//End text input
                });//End bottom_up display area
            });//End Window
    }
}
