use crate::{Config, Result};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use url::Url;
use urlencoding::decode;

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

    // write joins the host domain and path to make a file path
    // checks if the path is an existing file, if not it creates a new file
    // checks if the url contains %(s), if so decode it
    // writes the url to corresponding file and marks has_written bool as true
    pub fn write(&mut self, url: &Url) {
        // Todo: try to find a better way of doing the below 2 lines of code
        let base = url.host_str().unwrap_or("no host").to_string();
        let file_dir = self.path.join(&base);

        let mut url_file = self
            .url_files // referencing the UrlWriters HashMap
            .entry(base.to_owned()) // getting a value from the HashMap
            .or_insert_with(|| UrlFile::new(&file_dir)); // if a value is not found
                                                         // set it to file_dir

        let decoded_url = decode(url.as_str()).unwrap_or_default();
        writeln!(url_file.file, "{}", decoded_url).expect("could not write");

        url_file.has_written = true;
    }

    // aggregate_roots loops over url_files and joins k to self.path to make a file path
    // then it loops over the lines in the resulting file and inserts into hset
    // then it loops over the strings in hset and pushes the values into vhset
    // Config::new is called, passing the Vec of urls, save is called, then removes uncrawled
    pub fn aggregate_roots(self) -> Result<()> {
        let mut base_urls = Vec::new();

        for (k, _) in self.url_files.iter() {
            let file_dir = self.path.join(k);
            let file = File::open(file_dir)?;
            let mut uniques = HashSet::new();

            let unique_lines = BufReader::new(file)
                .lines()
                .map(|l| l.unwrap())
                .filter(|l| uniques.insert(l.to_owned()));

            base_urls.extend(unique_lines);
        }

        Config::new(base_urls, 4).save("config.toml")?;

        fs::remove_dir_all(self.path)?;

        Ok(())
    }
}
