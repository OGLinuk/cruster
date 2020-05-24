use crate::Result;
use reqwest;
use reqwest::header::CONTENT_TYPE;
use select::document::Document;
use select::predicate::Name;
use std::process::Command;
use url::Url;

pub struct Crawler {
    pub base: Url,
}

impl Crawler {
    pub fn new(url: Url) -> Self {
        Crawler { base: url }
    }

    pub fn from_url_string(str_url: &str) -> Result<Crawler> {
        Ok(Crawler::new(Url::parse(str_url)?))
    }

    // crawl sends request to Crawler.base url
    // parses the response, checks header content_type
    // finds <a> tags containing hrefs attributes (if utf-8) else defaults
    // maps the found hrefs to the base url
    // transforms/returns the resulting iterator to <Vec<Url>>
    pub fn crawl(&self, i: i32) -> Vec<Url> {
        let str_base = self.base.as_str();
        if let Ok(resp) = reqwest::get(str_base) {
            let c_type = resp
                .headers()
                .get(CONTENT_TYPE)
                .and_then(|h| Some(h.to_str()))
                .expect("could not get cont_type")
                .expect("could not expect c_type")
                .to_lowercase();

            if c_type.contains("utf-8") || c_type.contains("text/html") {
                println!("[{}] Fetching: {}", i, str_base);
                let doc = Document::from_read(resp).expect("could not read resp");
                let hrefs = doc.find(Name("a")).filter_map(|n| n.attr("href"));
                let full_urls = hrefs.filter_map(|url| self.base.join(&url).ok());
                full_urls.collect::<Vec<Url>>()
            } else if c_type.contains("application/pdf") {
                println!("[{}] Curling: {}", i, str_base);

                let mut pdf_name = str_base.split("/")
                    .last()
                    .expect("could not get last value of split")
                    .to_owned();

                pdf_name.insert_str(0, "pdfs/");

                let mut cmd = Command::new("curl");
                cmd.arg("-L")
                    .arg("--referer")
                    .arg(";auto")
                    .arg("-o")
                    .arg(pdf_name)
                    .arg(str_base)
                    .output()
                    .expect("could not curl");

                Default::default()
            } else {
                Default::default()
            }
        } else {
            Default::default()
        }
    }
}

