use is_url::is_url;
use walkdir::WalkDir;

use std::{
    fs::File,
    path::Path,
    borrow::Cow,
    error::Error,

    io::{
        Read,
        Write,
        BufRead,
        Result as IoResult,
    },
};

use zip::{
    CompressionMethod,

    write::{
        FileOptions,
        ExtendedFileOptions
    },
};

use sha2::{
    Digest,
    Sha256,
};

use crate::{
    args_cli::Flags,
    monset::vars::Vars,
    monset::macros::Macros,

    ui::{
        ui_base::UI,
        compress_alerts::CompressAlerts,
    },

    system::{
        pdf::Pdf,
        markdown::Markdown,
        reporting::Reporting,
    },
};

pub struct Tasks;

impl Tasks {

    pub async fn prints<R>(reader: R) -> Result<(), Box<dyn Error>> where R: BufRead, {
        let contents = reader.lines().collect::<Result<Vec<_>, _>>()?.join("\n");

        for line in contents.lines() {
            Vars::get_print(&line);
        }

        Ok(())
    }

    pub fn compress(contents: &str) -> IoResult<()> {
        if let Some(zip_file) = Vars::get_compress(contents) {
            UI::section_header("Compressing files", "normal");
            let folder_path = Vars::get_path(contents);
            
            let output_path = Path::new(&zip_file);
            let output_file = File::create(output_path)?;
            let mut zip = zip::ZipWriter::new(output_file);
            let options: FileOptions<ExtendedFileOptions> = FileOptions::default()
                .compression_method(CompressionMethod::Deflated)
                .compression_level(Some(9)) // Nível de compressão máximo
                .unix_permissions(0o755);

            for entry in WalkDir::new(&folder_path) {
                let entry = entry?;
                let path = entry.path();
                let name = path.strip_prefix(Path::new(&folder_path)).unwrap();

                if path.is_file() && !path.ends_with(".sha256") {
                    zip.start_file(name.to_str().unwrap(), options.clone())?;
                    let mut f = File::open(path)?;
                    let mut buffer = Vec::new();
                    f.read_to_end(&mut buffer)?;
                    zip.write_all(&buffer)?;

                    let file = path.to_str().unwrap();
                    CompressAlerts::added(&file, &zip_file);
                }
            }

            zip.finish()?;
            CompressAlerts::completed(&zip_file);
        }

        Ok(())
    }

    pub fn hash_sha256(file_path: &str) -> Result<String, Box<dyn Error>> {
        let mut file = File::open(file_path)?;
        let mut hasher = Sha256::new();

        let mut buffer = [0; 1024];
        
        loop {
            let bytes_read = file.read(&mut buffer)?;

            if bytes_read == 0 { break; }

            hasher.update(&buffer[..bytes_read]);
        }
    
        let hash = hasher.finalize();
        Ok(format!("{:x}", hash))
    }

    pub async fn download(contents: Option<&str>, url: &str, path: &str, flags: &Flags) -> Result<String, Box<dyn Error>> {
        let mut line_url = Cow::Borrowed(
            url.trim()
        );

        Reporting::check_download_errors(&line_url).await?;

        if !is_url(&line_url) { return Ok("".to_string()) }
    
        match Macros::handle_ignore_macro_flag(&line_url, flags.no_ignore) {
            Ok(new_line) => line_url = Cow::Owned(new_line),
            Err(_) => return Ok("".to_string()),
        }

        if let Some(contents) = contents {
            Markdown::create(&contents, &line_url, &path).await?;
        }

        Pdf::download_line(&line_url, url, path).await?;

        Ok("".to_string())
    }

}
