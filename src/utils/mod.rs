use reqwest::{
    blocking::Response,
    header::{HeaderMap, HeaderValue},
};

pub trait RequestGetter {
    fn get_xml(&self) {}

    fn get_request(&self, url: &str) -> Response {
        let client_builder = reqwest::blocking::Client::builder();

        let mut headers = HeaderMap::new();

        // Insert headers necessary to get it as personally relevant as possible
        headers.insert(
            "The-Timezone-IANA",
            HeaderValue::from_static("Africa/Johannesburg"),
        );

        let client_builder = client_builder.default_headers(headers);

        let client = client_builder.build().expect("Client unwrap fail");

        let res = client.get(url).send().expect("Request failed");

        if (!res.status().is_success()) {
            panic!("Request failed with status: {}", res.status());
        }

        res
    }
}
