pub struct RegExp;

impl RegExp {
    
    pub const EXTRACT_URL: &'static str = r"(?P<url>https?://[^\s]+)";
    pub const EXTRACT_PDF_NAME: &'static str = r"/([^/?]+)(?:\?.*)?$";
    pub const VALIDATE_EMAIL: &'static str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";

    pub const RENDER_EXTRA_QRCODE: &'static str = r#"\[!paimon qrcode data=['"](.*?)['"], size=(\d+)\]"#;

    pub const VALIDATE_TAGS: &'static str = r".*\*{1,2}.*|.*_.*|.*<\s*[a-zA-Z]+[^>]*>.*";

}
