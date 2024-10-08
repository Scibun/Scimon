pub struct CoreRegExp;

impl CoreRegExp {
     
    pub const EXTRACT_URL: &'static str = r"(?P<url>https?://[^\s]+)";
    pub const VALIDATE_TAGS: &'static str = r".*\*{1,2}.*|.*_.*|.*<\s*[a-zA-Z]+[^>]*>.*";

    pub const CLEAN_LINE: &'static str = r"\s*\{\s*\}\s*";

}
