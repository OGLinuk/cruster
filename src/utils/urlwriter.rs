use crate::{Result, Config};
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
    pub fn write(&mut self, url: &Url) -> Result<()> {
        let base = url.host_str().unwrap_or("no host").to_string();
        let file_dir = self.path.join(&base);
        
        // Todo use https://doc.rust-lang.org/std/io/struct.BufWriter.html instead
        // of just writeln! ~ see which way is faster (macro or BufWriter)
        let mut url_file = self
            .url_files
            .entry(base.to_owned())
            .or_insert_with(|| UrlFile::new(&file_dir));

        let str_url = url.as_str();
        if str_url.contains("%") {
            let decoded_url = decode(&str_url);
            writeln!(url_file.file, "{}", decoded_url.unwrap())?;
        } else {
            writeln!(url_file.file, "{}", url.as_str())?;
        }

        url_file.has_written = true;
        Ok(())
    }

    // aggregate_roots loops over url_files and joins k to self.path to make a file path
    // then it loops over the lines in the resulting file and inserts into hset
    // then it loops over the strings in hset and pushes the values into vhset
    // Config::new is called, passing the Vec of urls, save is called, then removes uncrawled
    pub fn aggregate_roots(self) -> Result<()> {
        let mut vhset = Vec::new();
        for (k, _v) in self.url_files.iter() {
            let file_dir = self.path.join(k);
            let mut hset = HashSet::new();
            let file = File::open(file_dir)?;

            for line in BufReader::new(file).lines() {
                hset.insert(line?);
            }
            for v in hset {
                vhset.push(v);
            }
        }

        let c = Config::new(vhset, 4);
        c.save(Path::new("config.toml"))?;

        fs::remove_dir_all(self.path)?;
        Ok(())
    }
}
