use regex::Regex;

use std::{
    io::BufRead,
    error::Error,

    process::{
        Stdio,
        Command,
    },
};

use crate::{
    ui::ui_base::UI,
    regexp::regex_core::CoreRegExp,
};

pub struct RunnerBlock;

impl RunnerBlock {
    
    pub fn exec_script(line: &str, program: &str) -> Result<(), Box<dyn Error>> {
        let line_cleanned = Regex::new(
            CoreRegExp::CLEAN_LINE
        ).unwrap().replace_all(
            &line, ""
        ).to_string();

        let output = Command::new(&program)
            .arg(line_cleanned)
            .stdout(Stdio::piped())
            .output()?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error executing Python script: {}", stderr);
        }

        Ok(())
    }

    pub async fn read_lines<R>(reader: R) -> Result<(), Box<dyn Error>> where R: BufRead {
        UI::section_header("Running");

        let contents = reader.lines().collect::<Result<Vec<_>, _>>()?.join("\n");
        let start_index = match (contents.find("commands {"), contents.find("commands{")) {
            (Some(idx1), Some(idx2)) => Some(idx1.min(idx2)),
            (Some(idx), None) | (None, Some(idx)) => Some(idx),
            (None, None) => None,
        };

        let end_index = contents.rfind("}");

        if let (Some(start_index), Some(end_index)) = (start_index, end_index) {
            let commands_content = &contents[start_index + "commands ".len()..end_index];

            for line in commands_content.lines() {
                let line_trimmed = line.trim();

                if line.trim().starts_with("commands {") {
                    continue;
                } else if line.trim().starts_with("}") {
                    break;
                }

                if line_trimmed.len() >= 3 {
                    if line_trimmed.ends_with(".py") {
                        Self::exec_script(&line_trimmed, "python")?;
                    } else if line_trimmed.ends_with(".js") {
                        Self::exec_script(&line_trimmed, "node")?;
                    } else {
                        eprintln!("Script não suportado: {}", line_trimmed);
                    }
                }
            }
        }

        Ok(())
    }

}
