use base64::prelude::*;

pub struct Base64;

impl Base64 {

    pub fn encode_html(content: &str) -> String {
        format!(
            "data:text/html;base64,{}", BASE64_STANDARD.encode(content)
        )
    }

}
