pub struct BlocksRegExp;

impl BlocksRegExp {
    
    pub const GET_README: [&'static str; 2] = [
        r"(?i)readme\s*\{",
        r"\}",
    ];

}
