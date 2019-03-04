use crate::Result;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use url::Url;

pub struct UrlWriter {
    pub path: PathBuf,
    url_files: HashMap<String, UrlFile>,
}

pub struct UrlFile {
    file: File,
    has_written: bool,
}

impl UrlFile {
    pub fn new(f: &Path) -> Self {
        UrlFile {
            file: File::create(f).expect("could not create file"),
            has_written: false,
        }
    }
}

impl UrlWriter {
    pub fn new(p: &Path) -> Self {
        fs::create_dir_all(p).expect("could not create all dirs");
        UrlWriter {
            path: p.into(),
            url_files: HashMap::new(),
        }
    }

    // write gets the host domain and joins it to the path
    // checks if the path is an existing file, if not it creates a new file
    // writes the given url to the file and marks has_written bool as true
    pub fn write(&mut self, url: &Url) -> Result<()> {
        let root = url.host_str().unwrap_or("no host").to_string();
        let file_dir = self.path.join(&root);
        let mut url_file = self
            .url_files
            .entry(root.to_owned())
            .or_insert_with(|| UrlFile::new(&file_dir));
        writeln!(url_file.file, "{}", url.as_str())?;
        url_file.has_written = true;
        Ok(())
    }
}
