use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::PathBuf;

pub trait InputSource {
    fn lines(&mut self) -> Box<dyn Iterator<Item = Result<String>> + '_>;
}

pub struct FileInput {
    reader: io::BufReader<File>,
}

impl FileInput {
    pub fn new(path: PathBuf) -> Result<Self> {
        let file = File::open(path)?;
        Ok(Self {
            reader: io::BufReader::new(file),
        })
    }
}

impl InputSource for FileInput {
    fn lines(&mut self) -> Box<dyn Iterator<Item = Result<String>> + '_> {
        Box::new(
            self.reader
                .by_ref()
                .lines()
                .map(|line| line.map_err(|e| e.into())),
        )
    }
}

pub struct StdinInput {
    reader: io::BufReader<io::Stdin>,
}

impl StdinInput {
    pub fn new() -> Self {
        Self {
            reader: io::BufReader::new(io::stdin()),
        }
    }
}

impl Default for StdinInput {
    fn default() -> Self {
        Self::new()
    }
}

impl InputSource for StdinInput {
    fn lines(&mut self) -> Box<dyn Iterator<Item = Result<String>> + '_> {
        Box::new(
            self.reader
                .by_ref()
                .lines()
                .map(|line| line.map_err(|e| e.into())),
        )
    }
}
