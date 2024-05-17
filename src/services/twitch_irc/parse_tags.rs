use std::{collections::HashMap, str::Split};

#[derive(Debug)]
pub struct TTags {
    pub client_nonce: String,
    pub color: String,
    pub display_name: String,
    pub emotes: String, // Use Vec for array
    pub first_message: bool,
    pub username: String,
    pub is_mod: bool,
    pub room_id: usize,
    pub is_subscriber: bool,
    pub user_id: String,
    pub user_type: String,
    pub badges: HashMap<String, bool>,
}

impl TTags {
    pub fn parse_into(data: String) -> Self {
        let splited = data.split(";");

        let mut tags = TTags {
            client_nonce: String::from(""),
            color: String::from(""),
            display_name: String::from(""),
            emotes: String::from(""),
            first_message: false,
            is_mod: false,
            room_id: 0,
            is_subscriber: false,
            user_id: String::from(""),
            user_type: String::from(""),
            badges: HashMap::new(),
            username: String::from(""),
        };

        splited.for_each(|element| {
            let (tag, value) = match element.split_once("=") {
                Some(v) => v,
                None => ("", ""),
            };

            match tag {
                "badges" => tags.badges = parse_badges(value),
                "emotes" => tags.emotes = String::from(value),
                "client-none" => tags.client_nonce = String::from(value),
                "color" => tags.color = String::from(value),
                "display-name" => {
                    tags.display_name = String::from(value);
                    tags.username = value.to_lowercase()
                }
                "first-msg" => tags.first_message = value == "1",
                "mod" => tags.is_mod = value == "1",
                "room-id" => tags.room_id = value.parse::<usize>().unwrap_or(0),
                "subscriber" => tags.is_subscriber = value == "1",
                "user-id" => tags.user_id = String::from(value),
                "user-type" => tags.user_type = String::from(value),
                _ => (),
            }
        });

        return tags;
    }
}

fn parse_badges(data: &str) -> HashMap<String, bool> {
    let badges = data.split(',');
    let mut decoded: HashMap<String, bool> = HashMap::new();

    badges.for_each(|e| {
        let (badge, status) = match e.split_once('/') {
            Some(v) => v,
            None => ("", ""),
        };
        decoded.insert(String::from(badge), status == "1");
    });

    return decoded;
}
