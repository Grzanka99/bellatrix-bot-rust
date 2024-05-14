use super::{
    parse_command::{ECommand, TParsedCommand},
    parse_source::TSource,
    parse_tags::TTags,
};

#[derive(Debug)]
pub struct TwitchIrcContext {
    pub itself: bool,
    pub command_type: ECommand,
    pub channel: String,
    pub command: TParsedCommand,
    pub message: Option<String>,
    pub tags: Option<TTags>,
    pub source: Option<TSource>,
}

trait IndexOf {
    fn index_of(&self, search: char, start: usize) -> Option<usize>;
    fn char_at(&self, idx: usize) -> &str;
}

impl IndexOf for String {
    fn index_of(&self, search: char, start: usize) -> Option<usize> {
        return match self.chars().skip(start).position(|c| c == search) {
            Some(v) => Some(v + start),
            None => None,
        };
    }

    fn char_at(&self, idx: usize) -> &str {
        return &self[idx..(idx + 1)];
    }
}

pub fn parse_message(raw_message: String) -> Option<TwitchIrcContext> {
    let mut idx = 0;
    let mut end_idx = 0;

    let mut raw_tags_component: Option<String> = None;
    let mut raw_source_component: Option<String> = None;
    let mut raw_command_component: Option<String> = None;
    let mut raw_parameters_component: Option<String> = None;

    match raw_message.char_at(idx) {
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

    match raw_message.char_at(idx) {
        ":" => {
            idx += 1;
            end_idx = match raw_message.index_of(' ', idx) {
                Some(v) => v,
                None => idx,
            };

            if end_idx <= idx {
                return None;
            }

            raw_source_component = Some(String::from(&raw_message[idx..(end_idx + 1)]));
            idx = end_idx + 1;
        }
        _ => (),
    }

    end_idx = match raw_message.index_of(':', idx) {
        Some(v) => v,
        None => raw_message.len(),
    };

    raw_command_component = Some(String::from(raw_message[idx..end_idx].trim()));

    if end_idx != raw_message.len() {
        idx = end_idx + 1;
        raw_parameters_component = Some(String::from(&raw_message[idx..(raw_message.len() - 1)]));
    }

    let tags = match raw_tags_component {
        Some(t) => Some(TTags::parse_into(t)),
        None => None,
    };

    let command = match raw_command_component {
        Some(v) => TParsedCommand::parse_into(v),
        None => unreachable!(),
    };

    let message = match (command.command, raw_parameters_component) {
        (ECommand::PRIVMSG, Some(v)) => Some(v.trim().to_string()),
        (_, _) => None,
    };

    return Some(TwitchIrcContext {
        itself: match tags {
            Some(ref v) => v.username == "bellabotrix",
            None => false,
        },
        command_type: command.command,
        channel: command.channel.clone(),
        command,
        tags,
        message,
        source: TSource::parse_into(raw_source_component),
    });
}
