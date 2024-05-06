use std::error::Error;

use crate::{
    utils::misc::Misc,
    ui::ui_base::PaimonUI,
    cmd::read_list::ReadList,
    monlib::api_get_list::ApiGetList,
};

pub struct Paimon;

impl Paimon {

    pub async fn get(run: &str, no_ignore: bool, no_comments: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> {
        if !run.starts_with("http") {
            ReadList::read_local_file(
                run, no_ignore, no_comments, kindle
            ).await?;
        } else {
            ReadList::read_remote_file(
                run, no_ignore, no_comments, kindle
            ).await?;
        }

        Ok(())
    }

    pub async fn core(run: &str, no_ignore: bool, no_comments: bool, kindle: Option<String>) {
        if !run.is_empty() {
            PaimonUI::header();
            
            if !Misc::check_is_user(run) {
                let _ = Paimon::get(
                    run, no_ignore, no_comments, kindle
                ).await;
            } else {
                let _ = ApiGetList::get(
                    run, no_ignore, no_comments, kindle
                ).await;
            }
        }
    }

}
