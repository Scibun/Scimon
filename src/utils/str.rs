pub struct StrUtils;

impl StrUtils {

    pub fn capitalize(s: &str) -> String {
        if s.is_empty() {
            return String::new();
        }

        let mut chars = s.chars();
        let first_char = chars.next().unwrap().to_uppercase().to_string();
        let rest: String = chars.collect();
        format!("{}{}", first_char, rest)
    }

}