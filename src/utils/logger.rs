use chrono::Utc;

fn get_now() -> String {
    return Utc::now().to_string()[11..19].to_string();
}

pub fn info(msg: String) {
    println!("{} [INFO] {}", get_now(), msg);
}

pub fn warning(msg: String) {
    println!("{} [WARN] {}", get_now(), msg);
}

pub fn error(msg: String) {
    println!("{} [ERRO] {}", get_now(), msg);
}
