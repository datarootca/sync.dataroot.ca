use async_trait::async_trait;

use crate::domain::article::{adapter::ArticleAdapter, model::{ArticleCreateModel}};
use chrono::{NaiveDateTime};
use serde::{Serialize, Deserialize};
use reqwest;

use serde_xml_rs::{from_str, Error};
use regex::Regex;
use url::Url;
use chrono::{ DateTime, Utc};
use chrono::format::ParseError;
fn remove_html_tags(input: &str) -> String {
    let re = Regex::new(r"<[^>]+>").unwrap();
    re.replace_all(input, "").to_string()
}

fn modify_medium_image_url(url_str: &str,fit: u16) -> String { 
    let url = match Url::parse(url_str) {
        Ok(url) => url,
        Err(_) => panic!("error"), // Invalid URL
    };

    // Check if the host is a Medium image URL
    if url.host_str() != Some("cdn-images-1.medium.com") {
        return url.to_string(); // Not a Medium image URL
    }
    
    let re = Regex::new(r"max\/[0-9]+").unwrap();
    return re.replace_all(url_str, "max/".to_owned() + &fit.to_string()).to_string()
}

fn parse_string_to_datetime(datetime_str: &str) -> Result<DateTime<Utc>, ParseError> {
    let datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S")?;
    Ok(DateTime::from_utc(datetime, Utc))
}

pub struct MediumArticleAdapter {
    // any necessary fields, such as an HTTP client
}

#[derive(Debug, Deserialize, Serialize)]
struct Rss {
    #[serde(rename = "channel")]
    channels: Vec<Channel>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Channel {
    title: String,
    description: String,
    image: Image,
    generator: String,
    #[serde(rename = "lastBuildDate", default)]
    last_build_date: String,
    #[serde(rename = "webMaster", default)]
    web_master: String,
    #[serde(rename = "item")]
    items: Vec<Item>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Image {
    url: String,
    title: String,
    #[serde(rename = "link")]
    image_link: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AtomLink {
    href: String,
    rel: String,
    #[serde(rename = "type")]
    link_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Item {
    title: String,
    guid: String,
    link: String,
    #[serde(rename = "category", default)]
    categories: Vec<String>,
    #[serde(rename = "pubDate", default)]
    publish_date: String,
    #[serde(rename = "atom_updated")]
    last_update: String,
    #[serde(rename = "content_encoded")]
    content: String,
}



async fn fetch_xml_rss(client: reqwest::Client,url: &str) -> Result<String, reqwest::Error> {
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    Ok(body)
}

fn parse_xml_rss(xml: &str) -> Result<Rss, Error> {
    let rss: Rss = from_str(xml)?;
    Ok(rss)
}
fn extract_medium_cdn_link(html: &str) -> Option<String> {
    // Regular expression pattern to match the src attribute of the img tag
    let pattern = r#"src="([^"]+)""#;
    let re = Regex::new(pattern).unwrap();

    // Find the first match in the HTML string
    if let Some(capture) = re.captures(html) {
        // Extract the captured group containing the link
        if let Some(link) = capture.get(1) {
            return Some(link.as_str().to_owned());
        }
    }
    None
}

impl MediumArticleAdapter {
    pub fn new(/* any necessary arguments */) -> Self {
        Self {
            // initialize fields
        }
    }
}
#[async_trait]
impl ArticleAdapter for MediumArticleAdapter {
    async fn fetch(&self, author: String) -> Result<Vec<ArticleCreateModel>, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let url = format!("https://medium.com/feed/@{}", &author);
        let xml = fetch_xml_rss(client,&url).await?;
        let processed_xml_str = xml
            .replace("content:", "content_")
            .replace("atom:updated","atom_updated");
        let rss = parse_xml_rss(&processed_xml_str).expect("Failed to parse XML RSS");
        let articles = rss.channels.into_iter().flat_map(|channel| channel.items)
            .into_iter().map(|article| {
            let cdn_link = extract_medium_cdn_link(&article.content).expect("Failed to extract link");
            let highres_link = modify_medium_image_url(&cdn_link, 800);
            let photo_link = modify_medium_image_url(&cdn_link, 600);
            let thumb_link = modify_medium_image_url(&cdn_link, 400);
            ArticleCreateModel::new(
                article.title,
                Some(article.content),
                article.guid,
                5,
                "medium".to_string(),
                article.link,
                author.to_owned(),
                parse_string_to_datetime(&article.publish_date).expect("Failed to extract date"),
                Some(highres_link),
                Some(photo_link),
                Some(thumb_link),
            )
        }).collect();
      
       Ok(articles)
    }
}
