//! Abstraction layer over `scraper` for better selection over tag atribbutes
//!
//! # The pain
//!
//! Querying HTML documents with `scraper` is much easier than
//! pattern matching strings and using regexes. But it's still
//! too low-level and not type-safe.
//!
//! Even after selecting we are still left with `ElementRef`
//! and all the context in the selector is lost. Further processing
//! needs to be made using atribbutes (strings) and values (also strings).
//!
//! # The dream
//!
//! The ideal solution would be a little type DSl that can
//! be used to define declarative queries. Much like a SQL
//! query builder.
//!
//! ```
//! let img = ImageSelector::new(&doc)
//!     .with_class("logo")
//!     .first();
//!
//! println!("{}", img.src());
//! ```
//!
//! This goes a little out of scope, but `Matcher` implements
//! a very inferior version of it:
//!
//! ```
//! let elems = MatcherBuilder::new()
//!    .select("img")
//!    .attr("class")
//!    .contains("logo")
//!    .build()
//!    .matches(&doc);
//! ```

use scraper::{ElementRef, Html, Selector};

pub struct Matcher {
    selector: Selector,
    targets: Vec<String>,
    attrs: Vec<String>,
}

impl Matcher {
    fn attr_matches(&self, elem: &ElementRef) -> bool {
        for attr in &self.attrs {
            if let Some(value) = elem.value().attr(&attr) {
                for target in &self.targets {
                    if value.contains(target) {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn parent_matches(&self, elem: &ElementRef) -> bool {
        let mut parent = elem.parent();

        while let Some(p) = parent {
            if let Some(parent_elem) = ElementRef::wrap(p) {
                if self.attr_matches(&parent_elem) {
                    return true;
                }
            }

            parent = p.parent()
        }

        false
    }

    pub fn matches<'a>(&self, document: &'a Html) -> impl Iterator<Item = ElementRef<'a>> {
        document
            .select(&self.selector)
            .filter(|elem| self.attr_matches(elem) || self.parent_matches(elem))
    }
}

pub struct MatcherBuilder {
    selector: String,
    targets: Vec<String>,
    attrs: Vec<String>,
}

impl MatcherBuilder {
    pub fn new() -> Self {
        Self {
            selector: String::new(),
            targets: Vec::new(),
            attrs: Vec::new(),
        }
    }

    pub fn select(mut self, selector: &str) -> Self {
        self.selector = selector.to_string();
        self
    }

    pub fn attr(mut self, attr: &str) -> Self {
        self.attrs.push(attr.to_string());
        self
    }

    pub fn contains(mut self, attr: &str) -> Self {
        self.targets.push(attr.to_string());
        self
    }

    pub fn build(self) -> Matcher {
        Matcher {
            selector: Selector::parse(&self.selector).unwrap(),
            targets: self.targets,
            attrs: self.attrs,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matcher() {
        let html = r#"
        <img id="logo"/>
        <img id="brand"/>
        <div class="logo">
            <img/>
        </div>
        <header>
            <div>
                <img src="/some.png"/>
                <img src="/logo.png"/>
            </div>
        </header>
        "#;

        let matcher = MatcherBuilder::new()
            .select("img")
            .attr("class")
            .attr("id")
            .attr("alt")
            .attr("src")
            .contains("logo")
            .contains("brand")
            .build();

        let doc = Html::parse_document(&html);
        let matches: Vec<_> = matcher.matches(&doc).collect();

        assert_eq!(matches.len(), 4)
    }
}
