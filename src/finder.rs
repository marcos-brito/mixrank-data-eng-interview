use std::time::Duration;
use reqwest::{
    IntoUrl, Url,
    blocking::{Client, ClientBuilder},
};
use scraper::Html;
use crate::{Site, query};

pub struct Finder {
    client: Client,
}

impl Finder {
    pub fn new() -> Self {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(5))
            .build()
            .expect("client creation should not fail");

        Self { client }
    }

    pub fn find<U: IntoUrl>(&self, target: U) -> Site {
        let target = match normalize_url(target.as_str()) {
            Some(url) => url,
            None => {
                log::warn!("can parse {} as url", target.as_str());

                return Site {
                    domain: target.as_str().to_string(),
                    logo: None,
                    favicon: None,
                }
            }
        };

        match self
            .client
            .get(target.to_string())
            .send()
            .and_then(|body| body.text())
        {
            Ok(body) => {
                let html = Html::parse_document(&body);
                let logos = query::img_tag(&html);
                let favicons = query::favicon(&html);

                Site {
                    domain: target.to_string(),
                    logo: logos.first().cloned(),
                    favicon: favicons.first().cloned(),
                }
            }
            Err(e) => {
                log::warn!("request failed for {}: {}", target.to_string(), e);

                Site {
                    domain: target.to_string(),
                    logo: None,
                    favicon: None,
                }
            }
        }
    }
}

fn normalize_url(raw: &str) -> Option<Url> {
    if let Ok(url) = Url::parse(raw) {
        return Some(url);
    }

    if let Ok(url) = Url::parse(&format!("https://{}", raw)) {
        return Some(url);
    }

    None
}
