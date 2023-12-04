use rocket::{get, response::Responder, serde::Deserialize, serde::Serialize};

use rocket_contrib::json::Json;
use scraper::{Html, Selector};

#[macro_use]
extern crate rocket;

// #[derive(Responder)]
// #[response(status = 500, content_type = "json")]
// struct ReleaseResponse {
//     inner
//     // #[response(ignore)]
//     // unrelated: MyType,
// }

fn to_float(s: &str) -> f64 {
    s.parse().unwrap()
}

// Implement Responder for Json<Vec<Release>>
impl<'r> Responder<'r, 'static> for Json<Vec<Release>> {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        to_float("asd");
        self.respond_to(self.request)
    }
}

#[get("/releases/today")]
async fn releasesForToday() -> Json<Vec<Release>> {
    let test = vec![Release {
        date: String::from("test"),
        title: String::from("test"),
    }];

    return Json(test);
}

#[get("/releases/popular")]
fn mostPopularWeeklyReleases() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    let r = rocket::build().mount("/", routes![releasesForToday]);

    r
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Release {
    title: String,
    date: String,
}

// #[derive(Responder)]
// pub struct ReleasesResponse {
//     inner: (Json<Vec<Release>>),
//     header: Header<'static>,
// }

pub struct Releases {}

// Helper function to retrieve the text value of a <td> element at a given index
fn get_td_text(row: &scraper::ElementRef, index: usize) -> String {
    row.select(&Selector::parse("td").unwrap())
        .nth(index)
        .map_or(String::new(), |td| td.text().collect())
}

async fn get_anime_for_today() -> Vec<Release> {
    let mut schedule_data: Vec<Release> = Vec::new();

    let response = get_request("https://subsplease.org/").await;

    let bdy = response.text().await.expect("Error unwrapping response");

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

    return schedule_data;
}
