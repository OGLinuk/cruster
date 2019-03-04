mod utils;
mod web;

use crate::utils::config::Config;
use crate::utils::urlwriter::UrlWriter;
use crate::web::crawler::Crawler;
use std::error::Error;
use std::path::Path;
//use std::fs::File;
use url::Url;

// Result replaces the return Result<(), Box<Error>> with just Result<T>
type Result<T> = std::result::Result<T, Box<Error>>;

// Checks if config.toml exists, if it does set cfg to the value
// if not it sets the default() and creates a config.toml file
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

    // File to write urls to be added to cfg.urls
    // Todo: Add aggregate function that walks root dir and reads/writes unique urls
    // to the to_crawl file
    //let to_crawl = File::create(&cfg.to_crawl);

    // Todo: change below to create a new crawler for each BaseUrl
    // as well as implement thread pooling
    let c = {
        let url = Url::parse(&cfg.urls[0])?;
        Crawler::new(url)
    };

    let urls = c.crawl()?;

    let mut url_writer = UrlWriter::new(Path::new("root"));

    for url in urls {
        url_writer.write(&url)?;
    }

    Ok(())
}
