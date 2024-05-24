pub struct BlocksRegExp;

impl BlocksRegExp {
    
    pub const GET_README_BLOCK: [&'static str; 2] = [
        r"(?i)readme\s*\{",
        r"\}",
    ];

    pub const GET_PATH_VAR: &'static str = r#"(?i)path\s*=\s*"([^"]+)""#;

    pub const GET_OPEN_VAR: &'static str = r#"(?i)open\s*=\s*"([^"]+)""#;

    pub const GET_README_VAR: &'static str = r#"(?i)readme\s*=\s*"([^"]+)""#;

    pub const GET_CHECKSUM_VAR: &'static str = r#"(?i)checksum\s*=\s*"([^"]+)""#;

}
