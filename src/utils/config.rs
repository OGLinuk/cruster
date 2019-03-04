use crate::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub threads: ThreadOptions,
    pub urls: Vec<String>,
    pub to_crawl: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ThreadOptions {
    pub max_workers: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            threads: ThreadOptions { max_workers: 4 },
            urls: vec!["https://en.wikipedia.org/wiki/Chaos_theory".into()],
            to_crawl: "to_crawl.txt".into(),
        }
    }
}

impl Config {
    pub fn load(f: &Path) -> Result<Config> {
        let conf = {
            let content = fs::read_to_string(f)?;
            toml::from_str(&content)?
        };
        Ok(conf)
    }

    pub fn save(&self, p: &Path) -> Result<()> {
        fs::write(p, toml::to_string_pretty(&self)?)?;
        Ok(())
    }
}
