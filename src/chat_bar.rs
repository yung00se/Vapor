use eframe::egui::{self, epaint, Button, Color32};
use emath::Align2;
use reqwest::header::SERVER;
use std::{env, io::{BufRead, BufReader, Write}, process::{Child, ChildStdin, ChildStdout, Command, Stdio}, sync::{Arc, Mutex}, time::Duration};
use std::net::TcpStream;

const SERVER_ADDR: &str = "192.168.56.1:6000";

pub struct Chat{
    username: String,
    chat_input: String,
    send_message: Arc<Mutex<Vec<String>>>,
    message_list: Arc<Mutex<Vec<String>>>,
}

impl Chat{
    pub fn new() -> Self{
        Self{
            username: "John".into(),
            chat_input: "".into(),
            send_message: Arc::new(Mutex::new(Vec::new())),
            message_list: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn start_client(&mut self, username: String) {
        let stream = Arc::new(Mutex::new(TcpStream::connect(SERVER_ADDR).unwrap()));
        let stream_clone = Arc::clone(&stream);
        let send_message_clone = Arc::clone(&self.send_message);
        let message_list_clone = Arc::clone(&self.message_list);
        std::thread::spawn(move || {
            let msg = send_message_clone.lock().unwrap().pop().unwrap();
            //eprintln!("here");
            //eprintln!("{}", msg);
            stream_clone.lock().unwrap().write_all(username.as_bytes()).expect("Failed to send username");
            let stream_result = TcpStream::connect(SERVER_ADDR);
            loop {
                let msg = send_message_clone.lock().unwrap().pop().unwrap();
                eprintln!("{}", msg);
                stream_clone.lock().unwrap().write_all(msg.as_bytes()).expect("Failed to send message to server");
            }
        // reader thread
        //std::thread::spawn(move || {

            //loop {

            //}
        });
    }
}

pub trait ChatBar{
    //fn send_message_getter(&self) -> String;
    //fn message_list_getter(&self) -> Vec<String>;
    fn display_chat_bar(&mut self, ctx: &egui::Context);
}

impl ChatBar for Chat {
    //fn send_message_getter(&self) -> String {
        //self.send_message.lock().unwrap().clone()
    //}

    //fn message_list_getter(&self) -> Vec<String> {
        //self.message_list.lock().unwrap().clone()
    //}
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
                            if send_button.clicked() {
                                let message = self.chat_input.clone();
                                self.send_message.lock().unwrap().push(message);
                            }
                        });//End text input
                });//End bottom_up display area
            });//End Window
    }
}