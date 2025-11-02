pub mod query;
pub mod matcher;

#[derive(PartialEq, Debug)]
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
