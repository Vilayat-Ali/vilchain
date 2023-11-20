#![allow(unused_assignments)]

use std::{fs, io, path::Path};

pub enum FileType {
    Wallet(String),
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

    pub fn gen_file(
        &self,
        file_type: FileType,
        filename: &str,
        contents: String,
    ) -> Result<(), io::Error> {
        let mut file_path: String = String::new();
        match file_type {
            FileType::Wallet(address) => {
                Self.gen_dir(format!("data/wallet/{address}"))?;
                file_path = format!("data/wallet/{address}/{}", filename);
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
        fs::write(file_path, contents)?;
        Ok(())
    }
}
