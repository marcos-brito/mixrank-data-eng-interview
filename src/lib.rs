pub mod matcher;
pub mod csv_writer;
pub mod query;
pub mod finder;
pub mod driver;

#[derive(PartialEq, Debug)]
pub struct Site {
    domain: String,
    logo: Option<Logo>,
    favicon: Option<Logo>,
}

impl Site {
    pub fn new(domain: String) -> Self {
        Self {
            domain,
            logo: None,
            favicon: None,
        }
    }

    pub fn to_csv(&self) -> String {
        format!(
            "{}, {}, {}",
            &self.domain,
            &self.logo.as_ref().map_or("null", |l| &l.url),
            &self.favicon.as_ref().map_or("null", |l| &l.url),
        )
    }
}

pub struct Logo {
    url: String,
    mime: Option<String>,
    width: Option<u64>,
    height: Option<u64>,
}

impl Logo {
    pub fn new(url: String) -> Self {
        Self {
            url,
            mime: None,
            width: None,
            height: None,
        }
    }
}
