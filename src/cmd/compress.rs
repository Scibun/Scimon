use walkdir::WalkDir;

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

    io::{
        self, 
        Read, 
        Write
    },
};

use crate::{
    syntax::vars_block::VarsBlock,

    ui::{
        ui_base::UI,
        compress_alerts::CompressAlerts,
    }
};

pub struct PathCompress;

impl PathCompress {

    pub fn compress(contents: &str) -> io::Result<()> {
        if let Some(zip_file) = VarsBlock::get_compress(contents) {
            UI::section_header("Compressing files");

            let folder_path = VarsBlock::get_path(contents);

            let output_path = Path::new(&zip_file);
            let output_file = File::create(output_path)?;

            let mut zip = zip::ZipWriter::new(output_file);
            let options: FileOptions<ExtendedFileOptions> = FileOptions::default()
                .compression_method(CompressionMethod::Deflated)
                .unix_permissions(0o755);

            for entry in WalkDir::new(&folder_path) {
                let entry = entry?;
                let path = entry.path();
                let name = path.strip_prefix(Path::new(&folder_path)).unwrap();

                if path.is_file() {
                    let file = name.to_str().unwrap();
                    CompressAlerts::added(&file, &zip_file);

                    zip.start_file(name.to_str().unwrap(), options.clone())?;

                    let mut f = File::open(path)?;
                    let mut buffer = Vec::new();

                    f.read_to_end(&mut buffer)?;
                    zip.write_all(&buffer)?;
                }
            }

            zip.finish()?;
            CompressAlerts::completed(&zip_file);
        }

        Ok(())
    }

}
