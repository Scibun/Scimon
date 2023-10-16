pub mod macros_flags {

    use std::error::Error;

    pub fn handle_comments(line: &str) -> Result<(), Box<dyn Error>> {
        if line.to_lowercase().contains(
            "!debug".trim()
        ) {
            eprintln!(
                "{}", &line.replace(
                    "!debug".trim(), ""
                ).replace(
                    "//", ""
                )
            )
        }

        return Ok(());
    }

    pub fn handle_ignore_macro_flag(line: &str, params: &str) -> Result<String, ()> {
        if params != "-noignore" && params != "--noignore" {
            if line.to_lowercase().contains("!ignore") {
                eprintln!(
                    "-> The file {} was ignored", line.replace(
                        " !ignore", ""
                    )
                );

                return Err(());
            }
        } else {
            return Ok(
                line.replace(
                    " !ignore", ""
                )
            );
        }
    
        Ok(line.to_string())
    }

}