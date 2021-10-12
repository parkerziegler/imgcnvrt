use std::env;
use std::path::Path;

pub struct Config {
    pub in_file_path: String,
    pub output_extension: String
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let in_file_path = match args.next() {
            Some(path) => {
                let p = Path::new(&path);

                if p.is_file() {
                    path
                } else {
                    return Err("The provided path does not pertain to a file on disk.")
                }
            },
            None => return Err("No input file path specified.")
        };

        let output_extension = match args.next() {
            Some(ext) => {
                match &ext[..] {
                    ".jpg" | ".jpeg" | ".png" | ".gif" | ".bmp" | ".ico" | ".tiff" | ".webp" | ".avif" | ".pnm" | ".dds" | ".tga" | ".exr" | ".ff" => ext,
                    _ => return Err("Unsupported file extension.")
                    
                }
            },
            None => return Err("No output extension specified.")
        };

        Ok(Config {
            in_file_path,
            output_extension
        })
    }
}