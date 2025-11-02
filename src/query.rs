use crate::Logo;
use scraper::{Html, Selector};

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
}
