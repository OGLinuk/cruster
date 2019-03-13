use crate::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub urls: Vec<String>,
    pub threads: ThreadOptions,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ThreadOptions {
    pub max_workers: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            urls: vec![
                "https://en.wikipedia.org/wiki/Chaos_theory".into(),
                "https://en.wikipedia.org/wiki/Parabola".into(),
            ],
            threads: ThreadOptions { max_workers: 4 },
        }
    }
}

impl Config {
    pub fn new(new_urls: Vec<String>, thread_workers: usize) -> Self {
        Config {
            urls: new_urls,
            threads: ThreadOptions {
                max_workers: thread_workers,
            },
        }
    }

    // Todo: look into <T: AsRef<Path>>(f: T)
    pub fn load<T: AsRef<Path>>(f: T) -> Result<Config> {
        let conf = {
            let content = fs::read_to_string(f)?;
            toml::from_str(&content)?
        };
        Ok(conf)
    }

    pub fn save<T: AsRef<Path>>(&self, p: T) -> Result<()> {
        fs::write(p, toml::to_string_pretty(&self)?)?;
        Ok(())
    }
}
