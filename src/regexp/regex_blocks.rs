pub struct BlocksRegExp;

impl BlocksRegExp {
    
    pub const GET_README: [&'static str; 2] = [
        r"(?i)readme\s*\{",
        r"\}",
    ];

    pub const GET_PATH: &'static str = r#"(?i)path\s*=\s*"([^"]+)""#;

    pub const GET_OPEN: &'static str = r#"(?i)open\s*=\s*"([^"]+)""#;

}
