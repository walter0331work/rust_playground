use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use std::error::Error;

#[derive(Debug)]
pub struct TonNumber {
    number: String,
    full_link: String,
    status: String,
    timer_text: String,
}

impl TonNumber {
    fn tokenize(&self) -> Vec<i32> {
        let r_number: Regex = Regex::new(r"(\d)").unwrap();
        let nums = r_number
            .find_iter(&self.number)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        nums
    }
}

trait DomUtils {
    fn get_text_content(&self) -> String;
    fn get_attr(&self, attr: &str) -> String;
    fn query_selector(&self, css_selector: &str) -> Option<Self>
    where
        Self: Sized;
    fn query_selector_all(&self, css_selector: &str) -> Vec<Self>
    where
        Self: Sized;
}
impl<'a> DomUtils for ElementRef<'a> {
    fn get_text_content(&self) -> String {
        let value = self.text().collect::<Vec<_>>().join("");

        value
    }

    fn get_attr(&self, attr: &str) -> String {
        let value = self.value().attr(attr).unwrap_or("").to_string();

        value
    }

    fn query_selector_all(&self, css_selector: &str) -> Vec<ElementRef<'a>> {
        if let Ok(selector) = Selector::parse(css_selector) {
            self.select(&selector)
                .into_iter()
                .collect::<Vec<ElementRef>>()
        } else {
            Vec::new()
        }
    }

    fn query_selector(&self, css_selector: &str) -> Option<ElementRef<'a>> {
        let list = self.query_selector_all(css_selector);
        let value = list.first().cloned();

        value
    }
}

pub async fn scrape(url: &str) -> Result<Vec<TonNumber>, Box<dyn Error>> {
    let res = reqwest::get(url).await?;
    let raw_html = res.text().await?;

    let document = Html::parse_document(&raw_html);
    let search_result_selector = Selector::parse(".js-search-results .tm-row-selectable")?;

    let result = document
        .select(&search_result_selector)
        .map(|record| {
            let path = record
                .query_selector("a")
                .map(|i| i.get_attr("href"))
                .unwrap_or(String::from(""));

            let full_link = format!("https://fragment.com{}", &path);

            let number = record
                .query_selector(".table-cell-value.tm-value")
                .map(|i| i.get_text_content())
                .unwrap_or(String::from(""));

            let timer_text = record
                .query_selector(".tm-timer")
                .map(|i| i.get_text_content())
                .unwrap_or(String::from(""));

            let status = record
                .query_selector(".table-cell-status-thin")
                .map(|i| i.get_text_content())
                .unwrap_or(String::from(""));

            TonNumber {
                number,
                full_link,
                status,
                timer_text,
            }
        })
        .collect::<Vec<TonNumber>>();

    Ok(result)
}

#[cfg(test)]
mod test {
    use crate::dom_utils::*;

    #[tokio::test]
    async fn test_parser() {
        let items = scrape("https://fragment.com/numbers?sort=ending")
            .await
            .unwrap();

        items.into_iter().for_each(|item| {
            println!("{:?}", &item);
            assert!(!item.full_link.is_empty());
        });
    }

    #[tokio::test]
    async fn test_tokenize() {
        let items = scrape("https://fragment.com/numbers?sort=ending")
            .await
            .unwrap();

        for n in items {
            assert!(n.tokenize().into_iter().len() > 0)
        }
    }
}

// struct DomQuery {
//     root_element: HTMLElement
// }

// trait DomQueryTrait {
//     fn select(&self, css_selector: &str) -> SelectResult,
// }

// impl DomQuery {
//     fn from_html(raw_html: &str) -> Self {
//         DomQuery { root_element: () }
//     }
// }

// struct HTMLElement {
//     text_content: String,
//     inner_html: String
// }
// //<div><a>content1</a></div> <div><a>conten2</a></div>

// struct SelectResult {
//     css_selector: &str,
//     data: Vec<HTMLElement>
// }

// trait SelectResultTrait {
//     fn value(&self) -> Vec<HTMLElement>;
//     fn text(&self) -> String;
// }

// impl SelectResultTrait for  SelectResult {
//     fn value(&self) -> Vec<HTMLElement> {
//         self.data
//     }
//     fn text(&self) -> String {
//         let value = String::from("");
//         self.value().into_iter().for_each(|element| {
//             value = value + &element.text_content;
//         });

//         value
//     }
// }
