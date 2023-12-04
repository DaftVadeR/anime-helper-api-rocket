use crate::utils;
use axum::Json;
use chrono::{Timelike, Utc};
use chrono_tz::Africa::Johannesburg;

use reqwest::Response;
use scraper::{Html, Selector};

use crate::Release;
use utils::RequestGetter;

struct UnprocessedRelease {
    title: String,
    time: String,
}

pub struct ReleasesController {}

impl RequestGetter for ReleasesController {}

impl ReleasesController {
    // Helper function to retrieve the text value of a <td> element at a given index
    fn get_td_text(row: &scraper::ElementRef, index: usize) -> String {
        row.select(&Selector::parse("td").unwrap())
            .nth(index)
            .map_or(String::new(), |td| td.text().collect())
    }
}

impl ReleasesController {
    async fn parse_request_for_release_info(response: Response) -> Vec<UnprocessedRelease> {
        let mut release_rows: Vec<UnprocessedRelease> = Vec::new();

        let bdy = response.text().await.expect("Error unwrapping response");

        println!("Body: {}", bdy);

        let document = Html::parse_document(&bdy);

        // Find the table with id "table-schedule"
        let table_selector =
            Selector::parse("#table-schedule").expect("Failed to create table selector");

        let table = document
            .select(&table_selector)
            .next()
            .expect("Table not found in document");

        let row_selector = Selector::parse("tr").expect("Failed to parse rows");

        for row in table.select(&row_selector) {
            let title = Self::get_td_text(&row, 0);
            let time = Self::get_td_text(&row, 1);

            release_rows.push(UnprocessedRelease { title, time });
        }

        return release_rows;
    }

    pub async fn get_anime_for_today() -> Vec<Release> {
        let response = ReleasesController::get_request("https://subsplease.org/").await;

        let unprocessed_releases: Vec<UnprocessedRelease> =
            Self::parse_request_for_release_info(response).await;

        return Self::get_processed_releases(unprocessed_releases);
    }

    fn get_processed_releases(unprocessed_releases: Vec<UnprocessedRelease>) -> Vec<Release> {
        let mut processed_releases: Vec<Release> = Vec::new();

        for release in unprocessed_releases {
            let mut now = chrono::Utc::now().with_timezone(&Johannesburg);

            let time_date = chrono::NaiveTime::parse_from_str(release.time.as_str(), "%H:%M")
                .expect("Could not parse time");

            now = now
                .with_hour(time_date.hour())
                .expect("Couldn't change hour");
            now = now
                .with_minute(time_date.minute())
                .expect("Couldn't change minute");

            let current_date_str = now.format("%Y-%m-%d %H:%M").to_string();

            processed_releases.push(Release {
                title: release.title,
                time_str: release.time,
                date_str: current_date_str,
                date: now.with_timezone(&Utc),
            });
        }

        return processed_releases;
    }
}

impl ReleasesController {
    pub async fn list_releases_today() -> Result<Json<Vec<Release>>, ()> {
        println!("->> {:<12} - list_releases", "HANDLER");
        let releases = Self::get_anime_for_today().await;

        Ok(Json(releases))
    }
}

impl ReleasesController {}
