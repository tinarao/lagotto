use crate::bangs::Bang;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Seek, Write},
    path::Path,
};

pub struct Storage {
    filepath: String,
    backup_filepath: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct StorageFile {
    bangs: Vec<Bang>,
}

impl StorageFile {
    pub fn new() -> Self {
        StorageFile { bangs: Vec::new() }
    }
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            filepath: "storage.json".to_string(),
            backup_filepath: "backup_storage.json".to_string(),
        }
    }

    fn read_to_struct(&self) -> Result<StorageFile, String> {
        self.validate_file_existense();
        self.backup()?;

        let storage: StorageFile;
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

        if let Err(e) = file.read_to_string(&mut contents) {
            return Err(e.to_string());
        }

        storage = serde_json::from_str(&contents).unwrap_or(StorageFile::new());
        return Ok(storage);
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
                panic!("failed to create a file: {}", e);
            }
        }
    }

    pub fn find_all(&self) -> Result<Vec<Bang>, String> {
        match self.read_to_struct() {
            Ok(s) => Ok(s.bangs),
            Err(e) => Err(e),
        }
    }

    pub fn find_bang(&self, alias: &String) -> Result<Bang, String> {
        self.validate_file_existense();
        let storage = match self.read_to_struct() {
            Ok(s) => s,
            Err(e) => {
                return Err(e);
            }
        };

        let bangs = storage.bangs;
        let bang = bangs.iter().find(|x| x.alias == alias.clone());
        match bang {
            Some(b) => return Ok(b.clone()),
            None => return Err(format!("bang {} is not set", alias)),
        }
    }

    // Basically, this method parses storage.json,
    // write it to StorageFile struct,
    // modifies it and writes back
    // TODO: check for possible optimization ways
    pub fn save_bang(&self, bang: &Bang) -> Result<(), String> {
        self.validate_file_existense();

        let mut storage: StorageFile;
        let mut file = self.get_file()?;

        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            return Err(e.to_string());
        }

        storage = serde_json::from_str(&contents).unwrap_or(StorageFile::new());
        storage.bangs.push((*bang).clone());

        self.write_updated_contents(storage)?;

        Ok(())
    }

    pub fn remove_bang(&self, alias: &String) -> Result<(), String> {
        self.validate_file_existense();
        match self.find_bang(&alias) {
            Ok(_) => {}
            Err(e) => return Err(e),
        };

        let mut storage = self.read_to_struct()?;
        let filtered: Vec<Bang> = storage
            .bangs
            .into_iter()
            .filter(|b| b.alias != alias.clone())
            .collect();

        storage.bangs = filtered;
        self.write_updated_contents(storage)?;
        return Ok(());
    }

    fn get_file(&self) -> Result<File, String> {
        OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.filepath)
            .map_err(|e| e.to_string())
    }

    fn write_updated_contents(&self, storage: StorageFile) -> Result<(), String> {
        let mut file = self.get_file()?;
        self.backup()?;
        self.clear_file()?;

        serde_json::to_writer_pretty(&mut file, &storage)
            .map_err(|e| format!("failed to write to storage file: {}", e))
    }

    fn clear_file(&self) -> Result<(), String> {
        let mut file = self.get_file()?;

        file.set_len(0)
            .map_err(|e| format!("failed to truncate file: {}", e))?;

        match file.rewind() {
            Ok(_) => {}
            Err(e) => {
                println!("failed to update the file: {e}. reverting changes.");
                self.apply_backup()?;
            }
        };

        return Ok(());
    }

    // Backups
    fn backup(&self) -> Result<(), String> {
        let backup = Path::new(&self.backup_filepath);

        match fs::copy(&self.filepath, backup) {
            Ok(_) => {}
            Err(e) => return Err(e.to_string()),
        };

        // voila, backup created

        return Ok(());
    }

    // Basically renames backup_storage.json to storage.json
    // So main storage file now is one that was backup.
    // Use it when something important fails, like clear_file, etc
    fn apply_backup(&self) -> Result<(), String> {
        match fs::remove_file(&self.filepath) {
            Ok(()) => {}
            Err(e) => {
                return Err(e.to_string());
            }
        };

        fs::rename(&self.backup_filepath, &self.filepath).map_err(|e| e.to_string())
    }
}
