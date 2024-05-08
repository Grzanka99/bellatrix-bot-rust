use std::{
    collections::HashMap,
    env,
    net::TcpStream,
    sync::mpsc::{channel, Sender},
    thread::{self, JoinHandle},
};

use websocket::{sync::Client, ClientBuilder, Message, OwnedMessage};

use crate::utils::logger;

use self::parser::parse_message;

mod parse_tags;
mod parser;

struct TwitchIrcContext {
    channel: String,
}

type EventHandler = fn(ctx: &mut TwitchIrcContext);

pub fn create_webstocket_client() -> Client<TcpStream> {
    return ClientBuilder::new("ws://irc-ws.chat.twitch.tv:80")
        .unwrap()
        .add_protocol("rust-websocket")
        .connect_insecure()
        .unwrap();
}

struct TwitchIrc {
    sender: Option<Sender<OwnedMessage>>,
    send_loop: Option<JoinHandle<()>>,
    receive_loop: Option<JoinHandle<()>>,
    handlers: HashMap<String, EventHandler>,
}

impl TwitchIrc {
    pub fn new() -> Self {
        return Self {
            sender: None,
            send_loop: None,
            receive_loop: None,
            handlers: HashMap::new(),
        };
    }

    pub fn start_loop(&mut self, client: Client<TcpStream>) {
        let (mut receiver, mut sender) = client.split().unwrap();

        let (tx, rx) = channel();
        let tx_1 = tx.clone();

        self.sender = Some(tx);

        self.send_loop = Some(thread::spawn(move || {
            loop {
                // Send loop
                let message = match rx.recv() {
                    Ok(m) => m,
                    Err(e) => {
                        println!("Send Loop: {:?}", e);
                        return;
                    }
                };
                match message {
                    OwnedMessage::Close(_) => {
                        let _ = sender.send_message(&message);
                        // If it's a close message, just send it and then return.
                        return;
                    }
                    _ => (),
                }
                // Send the message
                match sender.send_message(&message) {
                    Ok(()) => (),
                    Err(e) => {
                        println!("Send Loop: {:?}", e);
                        let _ = sender.send_message(&Message::close());
                        return;
                    }
                }
            }
        }));

        self.receive_loop = Some(thread::spawn(move || {
            for message in receiver.incoming_messages() {
                let message = match message {
                    Ok(m) => m,
                    Err(e) => {
                        logger::error(format!("Receive loop: {:?}", e));
                        let _ = tx_1.send(OwnedMessage::Close(None));
                        return;
                    }
                };
                match message {
                    OwnedMessage::Close(_) => {
                        let _ = tx_1.send(OwnedMessage::Close(None));
                    }
                    OwnedMessage::Ping(data) => match tx_1.send(OwnedMessage::Pong(data)) {
                        Ok(()) => (),
                        Err(err) => {
                            logger::error(format!("Receive loop: {:?}", err));
                        }
                    },
                    OwnedMessage::Text(msg) => {
                        // println!("Receive loop: {}", msg);
                        parse_message(msg);
                    }
                    _ => (),
                }
            }
        }));
    }

    pub fn send(&mut self, channel: &str, msg: &str) {
        match &self.sender {
            None => (),
            Some(sender) => {
                let _ = sender.send(OwnedMessage::Text(
                    format!("PRIVMSG {} :{}", channel, msg).to_string(),
                ));
            }
        }
    }

    fn join(&mut self, channel: &str) -> bool {
        if !channel.starts_with('#') {
            return false;
        }

        match &self.sender {
            None => return false,
            Some(sender) => {
                let res = sender.send(OwnedMessage::Text(format!("JOIN {}", channel)));

                match res {
                    Ok(()) => return true,
                    Err(_) => {
                        logger::error(format!("Cannot join channel: {}", channel));
                        return false;
                    }
                }
            }
        }
    }

    fn part(&mut self, channel: &str) -> bool {
        if !channel.starts_with('#') {
            return false;
        }

        match &self.sender {
            None => return false,
            Some(sender) => {
                let res = sender.send(OwnedMessage::Text(format!("PART {}", channel)));

                match res {
                    Ok(()) => return true,
                    Err(_) => {
                        logger::error(format!("Cannot part channel: {}", channel));
                        return false;
                    }
                }
            }
        }
    }

    pub fn connect(&mut self, username: &str, password: String) -> bool {
        match &self.sender {
            None => return false,
            Some(sender) => {
                let _ = sender.send(OwnedMessage::Text(
                    "CAP REQ :twitch.tv/membership twitch.tv/tags twitch.tv/commands".to_string(),
                ));

                let _ = sender.send(OwnedMessage::Text(format!("PASS {}", password,)));
                let _ = sender.send(OwnedMessage::Text(format!("NICK {}", username)));
                return true;
            }
        }
    }

    pub fn register_handler(&mut self, channel: &str, handler: EventHandler) -> bool {
        logger::info(format!("Joining room: {}", channel));
        let joined = self.join(channel);
        match joined {
            true => {
                self.handlers
                    .insert(channel.to_lowercase().to_string(), handler);
                logger::info(format!("Joined room: {}", channel));
                return true;
            }
            false => {
                logger::error(format!(
                    "Something went wrong while joining room: {}",
                    channel
                ));

                return false;
            }
        }
    }

    pub fn unregister_handler(&mut self, channel: &str) -> bool {
        logger::info(format!("Parting room: {}", channel));
        match self.part(channel) {
            true => {
                self.handlers.remove(channel);
                logger::info(format!("Parted room: {}", channel));
                return true;
            }
            false => {
                logger::error(format!(
                    "Something went wrong while parting room: {}",
                    channel
                ));

                return false;
            }
        }
    }
}

pub fn asd() {
    let mut client = TwitchIrc::new();

    match env::var("PASSWORD") {
        Err(_) => {
            print!("err");
        }
        Ok(password) => {
            println!("asdasd");
            client.start_loop(create_webstocket_client());
            client.connect("BellaBotrix", password);

            client.register_handler("#wannacry_tm", |ctx| {
                // (ctx.send)("siema".to_string());
                println!("AAAAAJ EM DE HAAANDLER");
            });

            let _ = client.send_loop.unwrap().join();
            let _ = client.receive_loop.unwrap().join();
        }
    };
}
