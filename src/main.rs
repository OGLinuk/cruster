mod utils;
mod web;

use crate::utils::config::Config;
use crate::utils::urlwriter::UrlWriter;
use crate::web::crawler::Crawler;
use std::error::Error;
use std::path::Path;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

// Result replaces the return Result<(), Box<Error>> with just Result<T>
type Result<T> = std::result::Result<T, Box<Error>>;

// main entrypoint to the program
fn main() {
    if let Some(e) = try_main().err() {
        eprintln!("Error: {}", e)
    }
}

// try_main checks if config.toml exists, if it does set cfg to the value
// if not it sets the default() and creates a config.toml file.
// Loops through urls, creates a Crawler, writes parsed url to crawled
// and sends the returned value from crawl to the tx channel.
// All urls on the rx channel are looped through and written to uncrawled
// and finally aggregate_roots is called
fn try_main() -> Result<()> {
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

    let mut raw_url_writer = UrlWriter::new(Path::new("uncrawled"));
    let mut parsed_url_writer = UrlWriter::new(Path::new("crawled"));

    /* Threadpooling implementation via threadpool::ThreadPool */
    let n_workers = cfg.threads.max_workers;
    let n_jobs = cfg.urls.len();
    let pool = ThreadPool::new(n_workers);
    let (tx, rx) = channel();

    for url in &cfg.urls {
        let tx = tx.clone();
        let c = Crawler::from_url_string(&url)?;
        parsed_url_writer.write(&c.base);

        pool.execute(move || {
            tx.send(c.crawl()).unwrap_or_default();
        });
    }

    /*
    First iteration
    for r in rx.iter().take(n_jobs) {
        for x in r {
            raw_url_writer.write(&x);
        }
    }

    Second iteration, but was never used ~ only serves as an example of a bad code smell
    Niamh line of code that *could* replace 5 lines of code above but is not as easily read
    rx.iter().take(n_jobs).for_each(|r| r.iter().for_each(|x| raw_url_writer.write(x)));

    above 5 and 1 line code chunks replaced with 3 lines below ~ code review with [Bahnahnah](https://github.com/Bahnahnah)
    .flatten() creates an iterator that flattens nested structure, removing the need for the second loop
    */

    for r in rx.iter().take(n_jobs).flatten() {
        raw_url_writer.write(&r);
    }
    /* End of threadpooling */

    // urlwriter::UrlWriter.aggregate_roots()
    raw_url_writer.aggregate_roots()?;

    Ok(())
}
