use reqwest::{
    header::{HeaderMap, HeaderValue},
    Response,
};

pub trait RequestGetter {
    fn get_xml() {}

    async fn get_request(url: &str) -> Response {
        let client_builder = reqwest::Client::builder();

        let mut headers = HeaderMap::new();

        // Insert headers necessary to get it as personally relevant as possible
        headers.insert(
            "The-Timezone-IANA",
            HeaderValue::from_static("Africa/Johannesburg"),
        );
        // Insert headers necessary to get it as personally relevant as possible
        headers.insert(
            "User-Agent",
            HeaderValue::from_static("Googlebot/2.1 (+http://www.google.com/bot.html)"),
        );

        let client_builder = client_builder.default_headers(headers);

        let client = client_builder.build().expect("Client unwrap fail");

        let res = client.get(url).send().await.expect("Request failed");

        if !res.status().is_success() {
            panic!("Request failed with status: {}", res.status());
        }

        res
    }
}
