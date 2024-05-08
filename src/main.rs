use dotenv::dotenv;
use services::twitch_irc::asd;
use utils::logger;

mod services;
mod utils;

fn main() {
    dotenv().ok();
    logger::info("get rekt".to_string());
    asd();
}
