use fancy_regex::Regex;
use scraper::{ElementRef, Html, Selector};
use std::error::Error;

#[derive(Debug)]
pub struct TonNumber {
    number: String,
    full_link: String,
    status: String,
    timer_text: String,
    timer_time: String,
}

impl TonNumber {
    fn tokenize(&self) -> Vec<i32> {
        let r_number: Regex = Regex::new(r"(\d)").unwrap();
        let nums = r_number
            .find_iter(&self.number)
            .map(|m| m.unwrap().as_str().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        nums
    }
}

// #[derive(Debug, Clone)]
struct DOMUtils<'a> {
    data: Vec<ElementRef<'a>>,
}

impl<'a> DOMUtils<'a> {
    fn new(element_ref: ElementRef<'a>) -> DOMUtils<'a> {
        DOMUtils {
            data: vec![element_ref],
        }
    }

    fn from_document(document: &'a Html) -> DOMUtils<'a> {
        let root_element = document.root_element();

        DOMUtils {
            data: vec![root_element],
        }
    }

    fn value(&self) -> &Vec<ElementRef<'a>> {
        &self.data
    }

    fn text(&self) -> String {
        self.data
            .to_owned()
            .into_iter()
            .fold("".to_string(), |acc: String, item| {
                acc + &item.text().into_iter().collect::<Vec<_>>().join("")
            })
    }

    fn attr(&self, name: &str) -> String {
        self.data[0].value().attr(name).unwrap_or("").to_string()
    }

    fn select(&self, css_selector: &str) -> DOMUtils<'a> {
        let selector = Selector::parse(css_selector).unwrap();
        let data = self.data[0]
            .select(&selector)
            .into_iter()
            .collect::<Vec<ElementRef>>();

        DOMUtils { data }
    }
}

pub async fn scrape(url: &str) -> Result<Vec<TonNumber>, Box<dyn Error>> {
    let res = reqwest::get(url).await?;
    let raw_html = res.text().await?;
    let document = Html::parse_document(&raw_html);
    let root_element = DOMUtils::from_document(&document);
    let result = root_element
        .select(".js-search-results .tm-row-selectable")
        .value()
        .into_iter()
        .map(|record| {
            let element_ref = *record;
            let path = DOMUtils::new(element_ref).select("a").attr("href");

            let full_link = format!("https://fragment.com{}", &path);

            let number = DOMUtils::new(element_ref)
                .select(".table-cell-value.tm-value")
                .text();

            let timer_element = DOMUtils::new(element_ref).select(".tm-timer");
            let timer_text = timer_element.text();
            let timer_time = timer_element.select("time").attr("datetime");

            let status = DOMUtils::new(element_ref)
                .select(".table-cell-status-thin")
                .text();

            TonNumber {
                number,
                full_link,
                status,
                timer_text,
                timer_time,
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
