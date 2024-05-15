pub struct MacrosRegExp;

impl MacrosRegExp {
    
    pub const GET_MACROS: &'static str = r"\s*![^\s]+";

    pub const GET_README: [&'static str; 2] = [
        "!readme",
        "!end_readme",
    ];

}
