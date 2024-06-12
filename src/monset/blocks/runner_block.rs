use std::{
    io::BufRead,
    error::Error,
};

use crate::{
    system::system::System,

    ui::{
        ui_base::UI,
        errors_commands_alerts::ErrorsCommandsAlerts,
    },
};

pub struct RunnerBlock;

impl RunnerBlock {

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
                        System::exec_script(&line_trimmed, "python")?;
                    } else if line_trimmed.ends_with(".js") {
                        System::exec_script(&line_trimmed, "node")?;
                    } else {
                        ErrorsCommandsAlerts::unsupported(&line_trimmed);
                    }
                }
            }
        }

        Ok(())
    }

}
