mod utils;
mod web;

use crate::utils::config::Config;
use crate::utils::urlwriter::UrlWriter;
use crate::web::crawler::Crawler;
use std::error::Error;
use std::path::Path;
use url::Url;

// Result replaces the return Result<(), Box<Error>> with just Result<T>
type Result<T> = std::result::Result<T, Box<Error>>;

// Checks if config.toml exists, if it does set cfg to the value
// if not it sets the default() and creates a config.toml file
// loops through urls, creates a Crawler, extracts urls from crawled
// writes crawled to parsed and every uncrawled to raw
fn main() -> Result<()> {
    let cfg = {
        let cfg_path = Path::new("config.toml");
        if cfg_path.exists() {
            Config::load(&cfg_path)?
        } else {
            let c = Config::default();
            c.save(&cfg_path)?;
            c
        }
    };

    let raw_path = Path::new("uncrawled");
    
    let mut raw_url_writer = UrlWriter::new(raw_path);
    let mut parsed_url_writer = UrlWriter::new(Path::new("crawled"));

    // Todo: implement thread pooling
    for url in &cfg.urls {
        let c = {
            let to_crawl = Url::parse(url)?;
            Crawler::new(to_crawl)
        };
        
        let urls = c.crawl()?;

        parsed_url_writer.write(&c.base)?;

        for uri in urls {
            raw_url_writer.write(&uri)?;
        }
    }

    raw_url_writer.aggregate_roots()?;

    Ok(())
}
