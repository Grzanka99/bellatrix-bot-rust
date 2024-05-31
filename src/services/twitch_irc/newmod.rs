use std::{
    env,
    net::TcpStream,
    thread::{self, JoinHandle},
};

use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};

use crate::services::twitch_irc::parser::TwitchIrcContext;

struct TwitchIrc {
    receive_loop: Option<JoinHandle<()>>,
}

fn create_and_connect_to_irc(
    username: String,
    password: String,
) -> WebSocket<MaybeTlsStream<TcpStream>> {
    let (mut socket, response) = connect("ws://irc-ws.chat.twitch.tv:80").expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    let _ = socket.send(Message::Text(
        "CAP REQ :twitch.tv/membership twitch.tv/tags twitch.tv/commands".to_string(),
    ));
    let _ = socket.send(Message::Text(format!("PASS {}", password)));
    let _ = socket.send(Message::Text(format!("NICK {}", username)));
    let _ = socket.send(Message::Text("JOIN #wannacry_tm".to_string()));

    return socket;
}

impl TwitchIrc {
    pub fn new() -> Self {
        return Self { receive_loop: None };
    }

    pub fn start_loop(&mut self, socket: WebSocket<MaybeTlsStream<TcpStream>>) {
        self.receive_loop = Some(thread::spawn(move || loop {
            let message = socket.read().expect("Error reading message");
            match TwitchIrcContext::parse_into(message.to_string()) {
                Some(v) => {
                    println!("{:?}", v);
                }
                None => (),
            }
        }));
    }
}

pub fn tungstenite_client_ex() {
    match env::var("PASSWORD") {
        Err(_) => (),
        Ok(password) => {
            let mut client = TwitchIrc::new(String::from("BellaBotrix"), password);
            client.start_loop();

            let _ = client.receive_loop.unwrap().join();
        }
    };
}
