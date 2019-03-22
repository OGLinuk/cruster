use crate::Result;
use reqwest;
use reqwest::header::CONTENT_TYPE;
use select::document::Document;
use select::predicate::Name;
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
                .expect("could not expect c_type");

            if c_type.to_lowercase().contains("utf-8") {
                println!("[{}] Fetching: {}", i, self.base.as_str());
                let doc = Document::from_read(resp).expect("could not read resp");
                let hrefs = doc.find(Name("a")).filter_map(|n| n.attr("href"));
                let full_urls = hrefs.filter_map(|url| self.base.join(&url).ok());
                full_urls.collect::<Vec<Url>>()
            } else {
                // Todo: implement functionality to download/save content that is not text/html
                Default::default()
            }
        } else {
            Default::default()
        }
    }
}

