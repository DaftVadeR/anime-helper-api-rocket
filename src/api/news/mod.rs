use crate::{utils, NEWS_API_BASE, NEWS_API_URI};
use axum::Json;

use crate::Release;
use chrono::{Timelike, Utc};
use reqwest::Response;
use scraper::{Html, Selector};
use serde::Deserialize;
use utils::RequestGetter;

use url::{ParseError, Url};

#[derive(Debug)]
struct UnprocessedArticle {
    title: String,
    utc_time: String,
    post_type: String,
    post_text: String,
    comments_url: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct Article {
    title: String,
    utc_time: String,
    post_type: String,
    post_text: String,
    comments_url: String,
    url: String,
}

enum PostType {
    Text,
    Link,
    Image,
    Gallery,
    Video,
    Embed,
    NA,
}

const REDDIT_DOMAINS: [&str; 2] = ["v.redd.it/", "www.reddit.com/"];

fn is_reddit_domain(url: &String) -> bool {
    for domain in REDDIT_DOMAINS {
        if url.contains(domain) {
            return true;
        }
    }

    return false;
}

fn get_post_type(post_type: String, p_text: Result<String, ()>, url: String) -> PostType {
    if post_type == "image" {
        PostType::Image
    } else if post_type == "hosted:video" {
        PostType::Video
    } else if post_type == "rich:video" {
        PostType::Embed
    } else if post_type == "gallery" {
        PostType::Gallery
    } else if post_type == "multi_media" && p_text != None {
        PostType::Text
    } else if post_type == "link" || !is_reddit_domain(url) {
        PostType::Link
    }

    return PostType::NA;
}

// #[derive(Debug, Deserialize, Serialize)]

// #[derive(Deserialize, Debug)]
// struct ApiRelease {
//     title: String,
//     page: String,
//     image_url: String,
//     time: String,
//     aired: bool,
// }

// // This `derive` requires the `serde` dependenctitle,
// #[derive(Deserialize, Debug)]
// struct ApiRequest {
//     tz: String,
//     schedule: Vec<ApiRelease>,
// }

#[derive(Debug)]
pub struct NewsController {}

impl RequestGetter for NewsController {}

// impl NewsController {
//     // Helper function to retrieve the text value of a <td> element at a given index
//     // fn get_td_text(row: &scraper::ElementRef, index: usize) -> String {
//     //     row.select(&Selector::parse("td").unwrap())
//     //         .nth(index)
//     //         .map_or(String::new(), |td| td.text().collect())
//     // }
// }

impl NewsController {
    fn get_paragraph_text_if_any(article: &scraper::ElementRef) -> String {
        let paragraph_selector = Selector::parse("div[slot=text-body]").unwrap();

        let paragraph: String = article
            .select(&paragraph_selector)
            .next()
            .map_or(String::new(), |p| p.text().collect());

        // match paragraph {
        //     Some(el) => el.map_or(String::new(), |p| p.text().collect()),
        //     None => None,
        // }

        paragraph
    }

    fn get_unprocessed_article_from_dom_data(article: &scraper::ElementRef) -> UnprocessedArticle {
        let title: String = article.attr("post-title").unwrap().to_string();
        let post_type: String = article.attr("post-type").unwrap().to_string();
        let utc_time: String = article.attr("created-timestamp").unwrap().to_string();
        let comments_url: String = article.attr("permalink").unwrap().to_string();
        let url: String = article.attr("content-href").unwrap().to_string();

        let post_text: String = Self::get_paragraph_text_if_any(article);

        UnprocessedArticle {
            url,
            title,
            post_type,
            comments_url,
            post_text,
            utc_time,
        }
    }

    async fn parse_reddit_page_from_response(response: Response) -> Vec<UnprocessedArticle> {
        let bdy = response.text().await.expect("Error unwrapping response");

        println!("Body: {}", bdy);

        let document = Html::parse_document(&bdy);

        let article_selector = Selector::parse("article > shreddit-post").unwrap();

        let articles = document.select(&article_selector);

        let mut converted: Vec<UnprocessedArticle> = Vec::new();

        for article in articles {
            converted.push(Self::get_unprocessed_article_from_dom_data(&article));
        }

        return converted;
    }

    pub async fn get_top_news_anime_reddit() -> Vec<Article> {
        // let response = NewsController::get_page_request(API_BASE).await;
        // let unprocessed_releases: Vec<UnprocessedRelease> =
        //     Self::parse_request_for_release_info(response).await;
        let response = NewsController::get_page_request(
            format!("{}{}", NEWS_API_BASE, NEWS_API_URI).as_str(),
            Some(true),
        )
        .await;

        let unprocessed_articles = Self::parse_reddit_page_from_response(response).await;

        Self::get_processed_news(unprocessed_articles);
        // return Self::get_processed_releases(unprocessed_releases);
    }

    //     fn parse_sp_latest_schedule(response: ApiRequest) -> Vec<Release> {
    //         let mut releases: Vec<Release> = Vec::new();

    //         for release in response.schedule {
    //             let mut now = chrono::Utc::now().with_timezone(&crate::DEFAULT_TIMEZONE);

    //             let time_date = chrono::NaiveTime::parse_from_str(release.time.as_str(), "%H:%M")
    //                 .expect("Could not parse time");

    //             now = now
    //                 .with_hour(time_date.hour())
    //                 .expect("Couldn't change hour");
    //             now = now
    //                 .with_minute(time_date.minute())
    //                 .expect("Couldn't change minute");

    //             let current_date_str = now.format("%Y-%m-%d %H:%M").to_string();

    //             releases.push(Release {
    //                 title: release.title,
    //                 aired: release.aired,
    //                 time_str: release.time,
    //                 slug: release.page.clone(),
    //                 url: format!("{}/{}", API_BASE, release.page),
    //                 image_url: format!("{}/{}", API_BASE, release.image_url),
    //                 date: now.with_timezone(&Utc),
    //             });
    //         }

    //         return releases;
    //     }

    //     // fn get_processed_releases(unprocessed_releases: Vec<UnprocessedRelease>) -> Vec<Release> {

    //     //     let mut processed_releases: Vec<Release> = Vec::new();

    //     //     for release in unprocessed_releases {
    //     //         let mut now = chrono::Utc::now().with_timezone(&crate::DEFAULT_TIMEZONE);

    //     //         let time_date = chrono::NaiveTime::parse_from_str(release.time.as_str(), "%H:%M")
    //     //             .expect("Could not parse time");

    //     //         now = now
    //     //             .with_hour(time_date.hour())
    //     //             .expect("Couldn't change hour");
    //     //         now = now
    //     //             .with_minute(time_date.minute())
    //     //             .expect("Couldn't change minute");

    //     //         // let current_date_str = now.format("%Y-%m-%d %H:%M").to_string();

    //     //         processed_releases.push(Release {
    //     //             title: release.title,
    //     //             time_str: release.time,
    //     //             date: now.with_timezone(&Utc),
    //     //             aired: false,
    //     //         });
    //     //     }

    //     //     return processed_releases;
    //     // }
}

impl NewsController {
    pub async fn list_news_reddit() -> Result<Json<Vec<Release>>, ()> {
        println!("->> {:<12} - list_news_reddit", "HANDLER");
        let releases = Self::get_top_news_anime_reddit().await;

        Ok(Json(releases))
    }
}

// impl NewsController {}
