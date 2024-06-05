use is_url::is_url;

use zip::{
    CompressionMethod,

    write::{
        FileOptions,
        ExtendedFileOptions
    },
};

use std::{
    fs::File,
    path::Path,
    borrow::Cow,
    error::Error,

    io::{
        Read, 
        Write, 
        Result as IoResult
    },
};

use crate::{
    args_cli::Flags,
    syntax::macros::Macros,
    syntax::vars_block::VarsBlock,

    ui::{
        ui_base::UI,
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
        compress_alerts::CompressAlerts,
    },

    system::{
        pdf::Pdf,
        hashes::Hashes,
        markdown::Markdown,
        reporting::Reporting,
        providers::Providers,
    },
};

pub struct Tasks;

impl Tasks {

    pub fn compress(contents: &str, files: &Vec<String>) -> IoResult<()> {
        if let Some(zip_file) = VarsBlock::get_compress(contents) {
            UI::section_header("Compressing files");
            let folder_path = VarsBlock::get_path(contents);

            let output_path = Path::new(&zip_file);
            let output_file = File::create(output_path)?;

            let mut zip = zip::ZipWriter::new(output_file);
            let options: FileOptions<ExtendedFileOptions> = FileOptions::default()
                .compression_method(CompressionMethod::Deflated)
                .unix_permissions(0o755);

            for file in files {
                let path = Path::new(file);
                let name = path.strip_prefix(Path::new(&folder_path)).unwrap();
    
                zip.start_file(name.to_str().unwrap(), options.clone())?;

                let mut f = File::open(path)?;
                let mut buffer = Vec::new();

                f.read_to_end(&mut buffer)?;
                zip.write_all(&buffer)?;

                CompressAlerts::added(
                    &file.replace(
                        &format!("{}/", &folder_path), ""
                    ), &zip_file
                );
            }

            zip.finish()?;
            CompressAlerts::completed(&zip_file);
        }

        Ok(())
    }

    pub async fn download(url: &str, path: &str, flags: &Flags) -> Result<String, Box<dyn Error>> {
        let mut line_url = Cow::Borrowed(
            url.trim()
        );

        Reporting::check_download_errors(&line_url).await?;

        if !is_url(&line_url) { return Ok("".to_string()) }
    
        match Macros::handle_ignore_macro_flag(&line_url, flags.no_ignore) {
            Ok(new_line) => line_url = Cow::Owned(new_line),
            Err(_) => return Ok("".to_string()),
        }

        Markdown::create(&line_url, &path).await?;

        if Pdf::is_pdf_file(&line_url).await? || Providers::check_provider_domain(url) && !line_url.contains(".md") {
            let result = Pdf::download(&line_url, path).await;
            
            match result {
                Ok(file) => {
                    let file_path = &format!("{}{}", &path, &file);
                    let password = Pdf::is_pdf_encrypted(&file_path);
                    let hash = Hashes::calculate_local_sha256(file_path)?;
                    
                    SuccessAlerts::download(&file, url, password, &hash);
                    return Ok(file_path.to_string())
                },

                Err(e) => ErrorsAlerts::download(e, url),
            }
        }

        Ok("".to_string())
    }

}
