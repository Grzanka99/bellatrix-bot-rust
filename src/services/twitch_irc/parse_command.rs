#[derive(Debug, Clone, Copy)]
pub enum ECommand {
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
pub struct TParsedCommand {
    pub command: ECommand,
    pub channel: String,
    pub is_cap_request_enabled: bool,
}

impl TParsedCommand {
    pub fn parse_into(raw_command_component: String) -> Self {
        let mut parsed_command = TParsedCommand {
            command: ECommand::UNKNOWN,
            channel: String::from(""),
            is_cap_request_enabled: false,
        };

        let mut command_parts = raw_command_component.split(" ");

        match command_parts.nth(0) {
            None => (),
            Some("JOIN") => {
                parsed_command.command = ECommand::JOIN;
                parsed_command.channel = String::from(command_parts.nth(0).unwrap_or(""));
            }
            Some("PART") => {
                parsed_command.command = ECommand::PART;
                parsed_command.channel = String::from(command_parts.nth(0).unwrap_or(""));
            }
            Some("NOTICE") => {
                parsed_command.command = ECommand::NOTICE;
                parsed_command.channel = String::from(command_parts.nth(0).unwrap_or(""));
            }
            Some("CLEARCHAT") => {
                parsed_command.command = ECommand::CLEARCHAT;
                parsed_command.channel = String::from(command_parts.nth(0).unwrap_or(""));
            }
            Some("HOSTTARGET") => {
                parsed_command.command = ECommand::HOSTTARGET;
                parsed_command.channel = String::from(command_parts.nth(0).unwrap_or(""));
            }
            Some("PRIVMSG") => {
                parsed_command.command = ECommand::PRIVMSG;
                parsed_command.channel = String::from(command_parts.nth(0).unwrap_or(""));
            }
            Some("USERSTATE") => {
                parsed_command.command = ECommand::USERSTATE;
                parsed_command.channel = String::from(command_parts.nth(0).unwrap_or(""));
            }
            Some("ROOMSTATE") => {
                parsed_command.command = ECommand::ROOMSTATE;
                parsed_command.channel = String::from(command_parts.nth(0).unwrap_or(""));
            }
            Some("001") => {
                parsed_command.command = ECommand::C001;
                parsed_command.channel = String::from(command_parts.nth(0).unwrap_or(""));
            }
            Some("PING") => {
                parsed_command.command = ECommand::PING;
            }
            Some("CAP") => {
                parsed_command.command = ECommand::CAP;
                parsed_command.is_cap_request_enabled = match command_parts.nth(1) {
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
}
