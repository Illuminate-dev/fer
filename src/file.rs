use anyhow::{anyhow, Result};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use std::fs::File as FsFile;

pub struct File {
    pub path: Option<PathBuf>,
    pub data: Vec<String>,
    pub modified: bool,
}

impl File {
    pub fn new(path: Option<PathBuf>) -> Self {
        Self {
            path,
            data: Vec::new(),
            modified: false,
        }
    }

    /// loads the file into memory - even if memory has unsaved changes
    pub fn load(&mut self) -> Result<()> {
        self.data.clear();

        if let Some(path) = self.path.as_ref() {
            let f = FsFile::open(path)?;
            let reader = BufReader::new(f);
            let mut lines = reader.lines();
            while let Some(line) = lines.next() {
                self.data.push(line?);
            }
        }
        Ok(())
    }

    pub fn insert_char(&mut self, c: char, y: usize, x: usize) -> Result<()> {
        self.data
            .get_mut(y)
            .ok_or(anyhow!("invalid row"))?
            .insert(x, c);
        self.modified = true;
        Ok(())
    }

    pub fn del_char(&mut self, y: usize, x: usize) -> Result<()> {
        self.data[y].remove(x as usize);
        self.modified = true;
        Ok(())
    }

    pub fn save(&mut self) -> Result<()> {
        if self.path.is_none() {
            return Err(anyhow!("no path"));
        }
        let path = self.path.as_ref().unwrap();
        let mut f = FsFile::create(path)?;
        f.write_all(self.data.join("\n").as_bytes())?;
        self.modified = false;
        Ok(())
    }

    pub fn get_path_name(&self) -> String {
        self.path
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or("No file path loaded!".to_string())
    }
}
