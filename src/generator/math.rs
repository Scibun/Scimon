use mathjax::MathJax;
use std::error::Error;

use crate::{
    syntax::vars::Vars,

    ui::{
        ui_base::UI,
        success_alerts::SuccessAlerts,
    },
};

pub struct Math {
    contents: Vec<(String, String)>,
}

impl Math {

    pub fn new(contents: &str) -> Self {
        Self {
            contents: Vars::get_all_math(contents),
        }
    }
    
    pub fn render(&self) -> Result<(), Box<dyn Error>> {
        let math_expressions = &self.contents;

        if !math_expressions.is_empty() {
            let renderer = MathJax::new()?;
            UI::section_header("math render", "normal");
            
            for (expression, file_name) in math_expressions {
                let result = renderer.render(&expression)?;
                let image = result.into_image(10.0)?;
                image.save(&file_name)?;

                SuccessAlerts::math(&file_name);
            }
        }

        Ok(())
    } 
    
}
