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

    // crawl sends request to Crawler.base
    // parses the response, checks header content_type
    // finds <a> tags containing hrefs attributes (if text/html)
    // maps the hrefs to the base url
    // transforms the resulting iterator to <Vec<Url>>
    pub fn crawl(&self) -> Result<Vec<Url>> {
        let mut resp = reqwest::get(self.base.as_str())?;
        let c_type =  resp.headers().get(CONTENT_TYPE)
                                    .and_then(|h| Some(h.to_str()))
                                    .unwrap_or(Ok(""))?;

        // Todo: find a better way to check below
        println!("{}", c_type);
        if c_type.contains("text/html;charset=UTF-8") ||
            c_type.contains("text/html;charset=utf-8") ||
             c_type.contains("text/html; charset=UTF-8") ||
             c_type.contains("text/html; charset=utf-8"){        
            println!("Fetching: {}", self.base.as_str());
            let doc = Document::from_read(resp)?;
            let hrefs = doc.find(Name("a")).filter_map(|n| n.attr("href"));
            let full_urls = hrefs.map(|url| self.base.join(url).expect("failed to join url"));
            Ok(full_urls.collect::<Vec<Url>>())
        } else {
            // Still working on below chunk 
            /*
            // Ran into no method found error
            // solution found here ~ https://stackoverflow.com/a/40392936
            let mut non_html = File::open(Path::new("non-html.txt")).expect("could not open file");
            let mut buf: Vec<u8> = vec![];
            resp.copy_to(&mut buf)?;
            //non_html.write(&buf)?;

            writeln!(non_html, "{:?}", &buf)?;
            */
            Ok(Default::default())
        }
    }
}
