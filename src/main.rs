use dotenv::dotenv;
use services::twitch_irc::asd;

mod services;
mod utils;

fn main() {
    dotenv().ok();
    asd();
}
