#![allow(unused_assignments)]

use std::{fs, io, path::Path};

pub enum FileType {
    Cred,
    Config,
    Log,
}

pub struct Filer;

impl Filer {
    pub fn gen_dir(&self, path: impl Into<String>) -> Result<(), io::Error> {
        let path: String = path.into();
        if !Path::exists(Path::new(&path)) {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }

    pub fn gen_file(file_type: FileType, filename: &str) -> Result<fs::File, io::Error> {
        let mut file_path: String = String::new();
        match file_type {
            FileType::Cred => {
                Self.gen_dir("data/secret")?;
                file_path = format!("data/secret/{}", filename);
            }
            FileType::Config => {
                Self.gen_dir("data/config")?;
                file_path = format!("data/config/{}", filename);
            }
            FileType::Log => {
                Self.gen_dir("data/log")?;
                file_path = format!("data/log/{}", filename);
            }
        }
        let f: fs::File = fs::File::create(file_path)?;
        Ok(f)
    }
}
