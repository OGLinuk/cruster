//use crate::Result;
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
    pub fn crawl(&self) -> Vec<Url> {
        if !Url::parse(self.base.as_str()).is_ok() {
            println!("Defaulted: {}", self.base.as_str());
            Default::default()
        };
        let resp = reqwest::get(self.base.as_str()).expect("could not set get req");
        let c_type =  resp.headers().get(CONTENT_TYPE)
                                    .and_then(|h| Some(h.to_str()))
                                    .expect("could not get cont_type");

        let un_c_type = c_type.expect("could not c_type");

        println!("{:?}", un_c_type);
        if un_c_type.contains("text/html"){        
            println!("Fetching: {}", self.base.as_str());
            let doc = Document::from_read(resp).expect("could not read resp");
            let hrefs = doc.find(Name("a")).filter_map(|n| n.attr("href"));
            let full_urls = hrefs.map(|url| self.base.join(url).expect("failed to join url"));
            full_urls.collect::<Vec<Url>>()
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
            Default::default()
        }
    }
}
