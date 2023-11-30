use rocket::{
    http::ContentType,
    response::{Responder, Response},
    serde::{Deserialize, Serialize},
};
use scraper::element_ref::Select;
use scraper::{Html, Selector};

use std::fs::File;
use std::io::BufReader;

use chrono::format::ParseError;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};

use xml::reader::{EventReader, XmlEvent};

#[macro_use]
extern crate rocket;

#[derive(Responder)]
#[response(status = 418, content_type = "json")]
struct ReleaseResponse(&'static str);

// #[rocket::async_trait]
// impl<'r> Responder<'r, 'static> for ReleaseResponse {
//     fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
//         Response::build()
//             .header(ContentType::Plain)
//             .sized_body(self.len(), Cursor::new(self))
//             .ok()
//     }
// }

#[get("/releases/today")]
fn releasesForToday() -> &'static str {
    let releases = get_anime_for_today();

    return "Done";

    // return Response {
    //     status: rocket::http::Status::Ok,
    //     headers: rocket::http::HeaderMap::new(),
    //     cookies: rocket::http::CookieJar::new(),
    //     body: rocket::http::Body::from_string("Hello, world!"),
    // };
}

#[get("/releases/popular")]
fn mostPopularWeeklyReleases() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![releasesForToday])
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Release {
    title: String,
    date: String,
}

pub struct Releases {}

// Helper function to retrieve the text value of a <td> element at a given index
fn get_td_text(row: &scraper::ElementRef, index: usize) -> String {
    row.select(&Selector::parse("td").unwrap())
        .nth(index)
        .map_or(String::new(), |td| td.text().collect())
}

// async fn get_request(url: &str) -> Result<reqwest::Response, reqwest::Error> {
async fn get_request(url: &str) -> reqwest::Response {
    let client = reqwest::Client::new();

    let res = client
        .get(url)
        .header(
            "The-Timezone-IANA",
            reqwest::header::HeaderValue::from_static("Africa/Johannesburg"),
        )
        .send()
        .await
        .unwrap();

    if !res.status().is_success() {
        panic!("Request failed with status: {}", res.status());
    }

    res
}

async fn get_anime_for_today() -> Vec<Release> {
    // let client: Client = reqwest::blocking::Client::new();

    // let response = reqwest::blocking::get("https://subsplease.org/")
    //     .unwrap()
    //     .text()
    //     .expect("Could not retrive Response");
    let mut schedule_data: Vec<Release> = Vec::new();

    let response = get_request("https://subsplease.org/").await;

    // // parse the HTML document
    // let doc_body = Html::parse_document(&response.text());

    // // select the table
    // let tables = Selector::parse("#schedule-table").unwrap();

    // for table in doc_body.select(&tables) {
    //     let titles = table.text().collect::<Vec<_>>();
    //     println!("{}", titles[0]);
    // }

    let bdy = response.text().await.expect("Error unwrapping response");

    // let body = response.text().await().expect("Failed to read response body");
    let document = Html::parse_document(&bdy);

    // Find the table with id "table-schedule"
    let table_selector = Selector::parse("#table-schedule").expect("Failed to parse for table");
    let table = document.select(&table_selector).next().unwrap();
    let row_selector = Selector::parse("tr").expect("Failed to parse rows");

    for row in table.select(&row_selector) {
        let title = get_td_text(&row, 0);
        let time = get_td_text(&row, 1);

        let mut current_date_str = chrono::Local::now().format("%Y-%m-%d").to_string();

        current_date_str.push_str(time.as_str());

        schedule_data.push(Release {
            title,
            date: current_date_str,
        });
    }

    // let ul = fragment.select(&ul_selector).next().unwrap();

    // for element in ul.select(&li_selector) {
    //     assert_eq!("li", element.value().name());
    // }
    // let table = document.select(&table_selector);

    // // Loop through the rows of the table
    // let on_something = |table: Select<'_, '_>| table.select(&Selector::parse("tr").unwrap());

    // for row in table.flat_map(on_something) {
    //     // Retrieve the text values of the first and second <td> elements
    //     let title = get_td_text(&row, 0);
    //     let time = get_td_text(&row, 1);

    //     // Push a tuple containing title and time into the collection
    //     schedule_data.push(Release { title, date: time });
    // }

    // // Loop through the collection and print each tuple
    // for release in schedule_data {
    //     println!("{} - {}", release.title, release.date);
    // }

    return schedule_data;
}
