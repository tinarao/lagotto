use crate::bangs::Bang;
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, Write},
    path::Path,
};

pub struct Storage {
    filepath: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct StorageFile {
    bangs: Vec<Bang>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            filepath: "storage.json".to_string(),
        }
    }

    pub fn validate_file_existense(&self) {
        if Path::new(&self.filepath).exists() {
            return;
        }

        match File::create(&self.filepath) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(b"{\n\n}") {
                    panic!("failed to create storage.json file: {}", e);
                }
            }
            Err(e) => {
                panic!("failde to create a file: {}", e);
            }
        }
    }

    pub fn find_bang(&self, alias: String) -> Result<Bang, String> {
        // 1. Parse file to StorageFile
        // 2. Find needed bang and return it
        self.validate_file_existense();
        let storage: StorageFile;

        let mut file = match OpenOptions::new().read(true).open(&self.filepath) {
            Ok(f) => f,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let mut contents_str = String::new();
        match file.read_to_string(&mut contents_str) {
            Ok(_) => {
                storage = match serde_json::from_str(&contents_str) {
                    Ok(v) => v,
                    Err(_) => StorageFile { bangs: Vec::new() },
                };
            }
            Err(e) => return Err(e.to_string()),
        };

        let bangs = storage.bangs;
        let bang = bangs.iter().find(|x| x.alias == alias);
        match bang {
            Some(b) => return Ok(b.clone()),
            None => return Err(format!("Bang {} is not set", alias)),
        }
    }

    // Basically, this method parses storage.json,
    // write it to StorageFile struct,
    // modifies it and writes back
    // TODO: check for possible optimization ways
    pub fn save_bang(&self, bang: &Bang) -> Result<(), String> {
        self.validate_file_existense();

        let mut storage: StorageFile;
        let mut file = match OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.filepath)
        {
            Ok(f) => f,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => {
                storage =
                    serde_json::from_str(&contents).unwrap_or(StorageFile { bangs: Vec::new() });
            }
            Err(e) => {
                return Err(format!("Failed to read storage file: {}", e));
            }
        }

        storage.bangs.push((*bang).clone());

        // seek to file start before writing
        file.set_len(0)
            .map_err(|e| format!("Failed to truncate file: {}", e))?;
        file.rewind()
            .map_err(|e| format!("Failed to rewind file: {}", e))?;

        serde_json::to_writer_pretty(&mut file, &storage)
            .map_err(|e| format!("Failed to write to storage file: {}", e))?;

        Ok(())
    }
}
