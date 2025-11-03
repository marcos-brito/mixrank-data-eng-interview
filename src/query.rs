//! Helper functions for querying HTML documents.
use crate::Logo;
use crate::matcher::MatcherBuilder;
use scraper::{Html, Selector};

/// Common trait for types that can find possible logos in an HTML document.
pub trait Query {
    fn run(&self, doc: &Html) -> Vec<Logo>;
}

impl<F> Query for F
where
    F: Fn(&Html) -> Vec<Logo> + 'static,
{
    fn run(&self, doc: &Html) -> Vec<Logo> {
        self(doc)
    }
}

/// Finds logos by trying every query in `queries` until success.
pub fn choice<T, I>(doc: &Html, queries: I) -> Vec<Logo> where T: Query, I: IntoIterator<Item = T> {
    for q in queries {
        let logos = q.run(doc);

        if logos.len() != 0 {
            return logos;
        }
    }

    Vec::new()
}

/// Finds logo canditates by comparing atribbutes.
///
/// It looks for any <img> tag with an id/class/src/alt
/// that contains either "logo" or "brand".
pub fn img_tag(doc: &Html) -> Vec<Logo> {
    let matcher = MatcherBuilder::new()
        .select("img")
        .attr("id")
        .attr("class")
        .attr("src")
        .attr("alt")
        .contains("logo")
        .contains("brand")
        .build();

    matcher
        .matches(&doc)
        .filter_map(|elem| {
            elem.value()
                .attr("src")
                .map(|value| Logo::new(value.to_string()))
        })
        .collect()
}

/// Finds favicons using <link> tags.
///
/// It looks for any <link> tags with an rel
/// that contains "icon".
pub fn favicon(doc: &Html) -> Vec<Logo> {
    let matcher = MatcherBuilder::new()
        .select("link")
        .attr("rel")
        .contains("icon")
        .build();

    matcher
        .matches(&doc)
        .filter_map(|elem| {
            elem.value()
                .attr("href")
                .map(|value| Logo::new(value.to_string()))
        })
        .collect()
}

/// Finds logo canditates using Open Graph metadata.
///
/// Any extra metadata (e.g. og:image:width, og:image:height) is associated
/// with the last seen og:image.
pub fn og_image(doc: &Html) -> Vec<Logo> {
    let selector = Selector::parse("meta[property^='og:image']").unwrap();
    let mut logos = Vec::new();
    let mut current: Option<Logo> = None;

    let pairs = doc
        .select(&selector)
        .filter_map(|elem| {
            Some((
                elem.value().attr("property")?,
                elem.value().attr("content")?,
            ))
        })
        .collect::<Vec<(&str, &str)>>();

    for (property, content) in pairs {
        match property {
            "og:image" => {
                if let Some(logo) = current {
                    logos.push(logo);
                }

                current = Some(Logo::new(content.to_string()));
            }
            "og:image:width" => {
                if let Some(logo) = current.as_mut() {
                    logo.width = content.parse().ok();
                }
            }
            "og:image:height" => {
                if let Some(logo) = current.as_mut() {
                    logo.height = content.parse().ok();
                }
            }
            _ => continue,
        }
    }

    if let Some(logo) = current {
        logos.push(logo);
    }

    logos
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_og_image() {
        let html = r#"
            <!DOCTYPE html>
            <head>
            <meta content="https://some/assets/logo.jpg" property="og:image">
            <meta content="64" property="og:image:height">
            <meta content="64" property="og:image:width">
            </head>
            <h1 class="foo">Hello, <i>world!</i></h1>
        "#;

        let expected = vec![Logo {
            url: "https://some/assets/logo.jpg".to_string(),
            mime: None,
            width: Some(64),
            height: Some(64),
        }];

        let got = og_image(&Html::parse_document(&html));

        for (l, r) in expected.iter().zip(got) {
            assert_eq!(l, &r)
        }
    }

    #[test]
    fn test_og_image_with_repetition() {
        let html = r#"
            <!DOCTYPE html>
            <head>
            <meta content="https://some/assets/logo.jpg" property="og:image">
            <meta content="64" property="og:image:height">
            <meta content="64" property="og:image:width">
            <meta content="https://some/assets/logo.jpg" property="og:image">
            <meta content="https://some/assets/logo.jpg" property="og:image">
            <meta content="64" property="og:image:width">
            </head>
            <h1 class="foo">Hello, <i>world!</i></h1>
        "#;

        let expected = vec![
            Logo {
                url: "https://some/assets/logo.jpg".to_string(),
                mime: None,
                width: Some(64),
                height: Some(64),
            },
            Logo {
                url: "https://some/assets/logo.jpg".to_string(),
                mime: None,
                width: None,
                height: None,
            },
            Logo {
                url: "https://some/assets/logo.jpg".to_string(),
                mime: None,
                width: Some(64),
                height: None,
            },
        ];

        let got = og_image(&Html::parse_document(&html));

        for (l, r) in expected.iter().zip(got) {
            assert_eq!(l, &r)
        }
    }

    #[test]
    fn test_favicon() {
        let html = r#"
            <!DOCTYPE html>
            <head>
            <link rel="icon" href="/favicon.svg"/>
            <link rel="icon" href="/favicon_dark.svg"/>
            <link rel="stylesheet" href="/main.css"/>
            </head>
        "#;

        let expected = vec![
            Logo {
                url: "/favicon.svg".to_string(),
                mime: None,
                width: None,
                height: None,
            },
            Logo {
                url: "/favicon_dark.svg".to_string(),
                mime: None,
                width: None,
                height: None,
            },
        ];

        let got = favicon(&Html::parse_document(&html));

        for (l, r) in expected.iter().zip(got) {
            assert_eq!(l, &r)
        }
    }
}
