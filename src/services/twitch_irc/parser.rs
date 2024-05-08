use crate::services::twitch_irc::parse_tags::parse_tags;

pub struct TwitchIrcContext {
    itself: bool,
    command_type: Option<String>,
    channel: String,
    command: Option<String>,
    message: Option<String>,
    tags: Option<String>,
    source: Option<String>,
}

trait IndexOf {
    fn index_of(&self, search: char, start: usize) -> Option<usize>;
}

impl IndexOf for String {
    fn index_of(&self, search: char, start: usize) -> Option<usize> {
        return self.chars().skip(start).position(|c| c == search);
    }
}

pub fn parse_message(raw_message: String) -> Option<TwitchIrcContext> {
    let mut idx = 0;
    let mut end_idx = 0;

    let mut raw_tags_component: Option<String> = None;
    let mut raw_source_component: Option<String> = None;
    let mut raw_command_component: Option<String> = None;
    let mut raw_parameters_component: Option<String> = None;

    match &raw_message[idx..idx + 1] {
        "@" => {
            end_idx = match raw_message.index_of(' ', 0) {
                Some(v) => v,
                None => end_idx,
            };
            raw_tags_component = Some(String::from(&raw_message[1..end_idx]));
            idx = end_idx + 1;
        }
        _ => (),
    }

    match &raw_message[idx..(idx + 1)] {
        ":" => {
            idx += 1;
            end_idx = match raw_message.index_of(' ', idx) {
                Some(v) => v,
                None => 0,
            };
            println!("{} {} {}", idx, end_idx, &raw_message[idx..(idx + 1)]);
            // raw_source_component = Some(String::from(&raw_message[idx..(end_idx + 1)]));
            idx = end_idx + 1;
        }
        _ => (),
    }

    end_idx = match raw_message.index_of(' ', idx) {
        Some(v) => v,
        None => raw_message.len(),
    };

    // raw_command_component = Some(String::from(raw_message[idx..end_idx].trim()));
    //
    // if end_idx != raw_message.len() {
    //     idx = end_idx + 1;
    //     raw_parameters_component = Some(String::from(&raw_message[idx..raw_message.len()]));
    // }
    //
    // let command = match raw_command_component {
    //     Some(v) => parse_command_component(v),
    //     None => unreachable!(),
    // };
    //

    // println!("{:?},", raw_message);

    println!("raw sou: {:?}", raw_source_component);
    println!("raw par: {:?}", raw_parameters_component);

    // println!("cmd: {:?}", command);

    let tags = match raw_tags_component {
        Some(t) => Some(parse_tags(t)),
        None => None,
    };

    return None;
}

#[derive(Debug)]
enum ECommand {
    JOIN,
    PART,
    NOTICE,
    CLEARCHAT,
    HOSTTARGET,
    PRIVMSG,
    PING,
    CAP,
    GLOBALUSERSTATE,
    USERSTATE,
    ROOMSTATE,
    RECONNECT,
    // C421,
    C001,
    // C002,
    // C003,
    // C004,
    // C353,
    // C366,
    // C372,
    // C375,
    // C376,
    UNKNOWN,
}

#[derive(Debug)]
struct TParsedCommand {
    command: ECommand,
    channel: String,
    is_cap_request_enabled: bool,
}

fn parse_command_component(raw_command_component: String) -> TParsedCommand {
    let mut parsed_command = TParsedCommand {
        command: ECommand::UNKNOWN,
        channel: String::from(""),
        is_cap_request_enabled: false,
    };

    let mut command_parts = raw_command_component.split(" ");

    command_parts.clone().for_each(|e| {
        println!("{:?}", e);
    });

    match command_parts.nth(0) {
        None => (),
        Some("JOIN") => {
            parsed_command.command = ECommand::JOIN;
            parsed_command.channel = String::from(command_parts.nth(1).unwrap_or(""));
        }
        Some("PART") => {
            parsed_command.command = ECommand::PART;
            parsed_command.channel = String::from(command_parts.nth(1).unwrap_or(""));
        }
        Some("NOTICE") => {
            parsed_command.command = ECommand::NOTICE;
            parsed_command.channel = String::from(command_parts.nth(1).unwrap_or(""));
        }
        Some("CLEARCHAT") => {
            parsed_command.command = ECommand::CLEARCHAT;
            parsed_command.channel = String::from(command_parts.nth(1).unwrap_or(""));
        }
        Some("HOSTTARGET") => {
            parsed_command.command = ECommand::HOSTTARGET;
            parsed_command.channel = String::from(command_parts.nth(1).unwrap_or(""));
        }
        Some("PRIVMSG") => {
            parsed_command.command = ECommand::PRIVMSG;
            parsed_command.channel = String::from(command_parts.nth(1).unwrap_or(""));
        }
        Some("USERSTATE") => {
            parsed_command.command = ECommand::USERSTATE;
            parsed_command.channel = String::from(command_parts.nth(1).unwrap_or(""));
        }
        Some("ROOMSTATE") => {
            parsed_command.command = ECommand::ROOMSTATE;
            parsed_command.channel = String::from(command_parts.nth(1).unwrap_or(""));
        }
        Some("001") => {
            parsed_command.command = ECommand::C001;
            parsed_command.channel = String::from(command_parts.nth(1).unwrap_or(""));
        }
        Some("PING") => {
            parsed_command.command = ECommand::PING;
        }
        Some("CAP") => {
            parsed_command.command = ECommand::CAP;
            parsed_command.is_cap_request_enabled = match command_parts.nth(2) {
                Some("ACL") => true,
                _ => false,
            }
        }
        Some("GLOBALUSERSTATE") => {
            parsed_command.command = ECommand::GLOBALUSERSTATE;
        }
        Some("RECONNECT") => {
            parsed_command.command = ECommand::RECONNECT;
        }
        Some(_) => {
            parsed_command.command = ECommand::UNKNOWN;
        }
    }

    return parsed_command;
}
