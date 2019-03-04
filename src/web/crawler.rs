use crate::Result;
use reqwest;
use select::document::Document;
use select::predicate::Name;
use url::Url;

pub struct Crawler {
    base: Url,
}

impl Crawler {
    pub fn new(url: Url) -> Self {
        Crawler { base: url }
    }

    // crawl sends a request to the Crawler.base url
    // parses the response
    // finds <a> tags containing hrefs
    // maps the hrefs to the base url
    // transforms the resulting iterator to <Vec<Url>>
    pub fn crawl(&self) -> Result<Vec<Url>> {
        let resp = reqwest::get(self.base.as_str())?;
        let doc = Document::from_read(resp)?;
        let hrefs = doc.find(Name("a")).filter_map(|n| n.attr("href"));
        let full_urls = hrefs.map(|url| self.base.join(url).expect("failed to join url"));
        Ok(full_urls.collect::<Vec<Url>>())
    }
}
