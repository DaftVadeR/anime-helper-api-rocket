use reqwest::{
    header::{HeaderMap, HeaderValue},
    Response,
};
use async_trait::async_trait;
use http_cache_reqwest::{Cache, CacheMode, CACacheManager, HttpCache, HttpCacheOptions};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, Result, ClientWithMiddleware};

#[async_trait]
pub trait RequestGetter {
    fn get_xml() {}
    
    async fn get_base_client() -> Client {
        let mut headers = HeaderMap::new();

        // Make dynamic maybe?
        headers.insert(
            "The-Timezone-IANA",
            HeaderValue::from_static("Africa/Johannesburg"),
        );

        // Make sure site is giving us the real data.
        headers.insert(
            "User-Agent",
            HeaderValue::from_static("Googlebot/2.1 (+http://www.google.com/bot.html)"),
        );

        let client_builder = reqwest::Client::builder();

        let inner_client = client_builder.default_headers(headers).build().unwrap();

        inner_client
    }
    
    async fn get_cached_client() -> ClientWithMiddleware {
        let client = Self::get_base_client().await;
        
        let client_builder = ClientBuilder::new(client);
        
        client_builder 
                .with(Cache(HttpCache {
                    mode: CacheMode::Default,
                    manager: CACacheManager::default(),
                    options: HttpCacheOptions::default(),
                }))
                .build()
    }


    async fn get_uncached_client() -> ClientWithMiddleware {
        let client = Self::get_base_client().await;
 
        let client_builder = ClientBuilder::new(client);
        return client_builder.build();
    }

    async fn get_page_request(url: &str, is_cached: Option<bool>) -> Response {
        let client = Self::get_cached_client().await;

        let res = client.get(url).send().await.expect("Request failed");

        if !res.status().is_success() {
            panic!("Request failed with status: {}", res.status());
        }

        res
    }

    async fn get_json_request(url: &str, is_cached: Option<bool>) -> Response {
        let use_cache = is_cached.unwrap_or(false);
        
        let client_builder = reqwest::Client::builder();

        let mut headers = HeaderMap::new();

        headers.insert("Accepts", HeaderValue::from_static("application/json"));

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
