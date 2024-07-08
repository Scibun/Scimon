use std::env::consts::OS as OS;

pub struct Plataforms;

impl Plataforms {

    pub fn get_os() -> String {
        OS.to_string()
    }

    pub fn get_bin_name(name: &str) -> String {
        let os = Self::get_os();

        if name == "python" {
            if os == "windows" {
                "python".to_string()
            } else {
                "python3".to_string()
            }
        } else {
            name.to_string()
        }
    }

}
