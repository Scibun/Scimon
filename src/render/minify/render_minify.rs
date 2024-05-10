use regex::Regex;

use crate::render::minify::render_minify_regex::RenderMarkdownMinifyRegExp;

pub struct RenderMarkdownMinify;

impl RenderMarkdownMinify {

    pub fn js(code: &str) -> String {
        let code = Regex::new(RenderMarkdownMinifyRegExp::MIN_JS_REMOVE_WHITESPACE).unwrap().replace_all(code, " ");
        let code = Regex::new(RenderMarkdownMinifyRegExp::MIN_JS_REMOVE_SINGLE_LINE_COMMENT).unwrap().replace_all(&code, "");
        let code = Regex::new(RenderMarkdownMinifyRegExp::MIN_JS_REMOVE_MULTI_LINE_COMMENT).unwrap().replace_all(&code.trim(), "");
        let code = Regex::new(RenderMarkdownMinifyRegExp::MIN_JS_REMOVE_STRINGS).unwrap().replace_all(&code, "\"\"");
        let code = Regex::new(RenderMarkdownMinifyRegExp::MIN_JS_REMOVE_OPERATORS_KEYWORDS).unwrap().replace_all(&code, "$1");
        let code = Regex::new(RenderMarkdownMinifyRegExp::MIN_JS_REMOVE_SPACES).unwrap().replace_all(&code, "$1");

        let code = Regex::new(
            &format!(r"\b({})\b", RenderMarkdownMinifyRegExp::MIN_JS_KEYWORDS)
        ).unwrap().replace_all(
            &code, " $1 "
        );

        let code = Regex::new(RenderMarkdownMinifyRegExp::MIN_JS_DUPLICATE_SPACES).unwrap().replace_all(&code, " ");
        let code = Regex::new(RenderMarkdownMinifyRegExp::MIN_JS_LOGICAL_OPERATORS).unwrap().replace_all(&code, "||");
        let code = Regex::new(RenderMarkdownMinifyRegExp::MIN_JS_WHITESPACE_TRIM).unwrap().replace_all(&code, "$1");

        let code = Regex::new(RenderMarkdownMinifyRegExp::MIN_JS_DOUBLE_QUOTED_STRING).unwrap().replace_all(
            &code, |caps: &regex::Captures| {
                let inner = &caps[0][1..caps[0].len() - 1];
                format!("\"{}\"", inner.replace("\\\"", "\""))
            }
        );

        code.to_string()
    }

    pub fn css(code: &str) -> String {
        let css = Regex::new(RenderMarkdownMinifyRegExp::MIN_CSS_REMOVE_MULTI_LINE_COMMENT).unwrap().replace_all(code, "");
        let css = Regex::new(RenderMarkdownMinifyRegExp::MIN_CSS_REMOVE_WHITESPACE).unwrap().replace_all(&css, " ");
        let css = Regex::new(RenderMarkdownMinifyRegExp::MIN_CSS_REMOVE_SPACES).unwrap().replace_all(&css, "$1");
        css.to_string()
    }
   
}
