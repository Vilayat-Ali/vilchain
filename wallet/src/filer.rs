use std::{fs, io, path::Path};

pub enum FileType {
    Cred,
    Config,
    Log,
}

pub struct Filer;

impl Filer {
    pub fn gen_file(file_type: FileType, filename: &str) -> Result<fs::File, io::Error> {
        match file_type {
            FileType::Cred => {
                if !Path::new("data/secrets").exists() {
                    fs::create_dir_all("data/secrets")?;
                }
                let f = fs::File::create(format!("data/secrets/{}", filename))?;
                Ok(f)
            }
            FileType::Config => {
                if !Path::new("data/secrets").exists() {
                    fs::create_dir_all("/data/config")?;
                }
                let f = fs::File::create(format!("data/configs/{}", filename))?;
                Ok(f)
            }
            FileType::Log => {
                if !Path::new("data/secrets").exists() {
                    fs::create_dir_all("data/logs")?;
                }
                let f = fs::File::create(format!("data/logs/{}", filename))?;
                Ok(f)
            }
        }
    }
}
