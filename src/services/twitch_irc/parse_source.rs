#[derive(Debug)]
pub struct TSource {
    pub username: Option<String>,
    pub host: String,
}

impl TSource {
    pub fn parse_into(raw_source: Option<String>) -> Option<Self> {
        return match raw_source {
            None => None,
            Some(v) => {
                let source_parts: Vec<&str> = v.split("!").collect();

                let username = match source_parts.len() {
                    2 => Some(source_parts[0].to_string()),
                    _ => None,
                };

                let host = match source_parts.len() {
                    2 => source_parts[1].trim().to_string(),
                    _ => source_parts[0].trim().to_string(),
                };

                return Some(TSource { username, host });
            }
        };
    }
}
