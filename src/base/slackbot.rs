use std::collections::HashMap;

use reqwest::header::HeaderMap;
use serde_json::{Value, Map};

use super::http::HttpClient;

pub struct SlackHttpClient {
    http_client: HttpClient,
    slackbot: String,
}

#[allow(dead_code)]
impl SlackHttpClient {
    pub fn new(slackbot: &str) -> Self {
        let http_client = HttpClient::new();
        Self {
            http_client: http_client,
            slackbot: String::from(slackbot),
        }
    }

    fn package_text(&self, sender: &str, content: &str) -> String {
        return format!("{sender}:\n{content}")
    }

    pub async fn send_text(&self, sender: &str, content: &str) {
        let mut params: HashMap<String, Value> = HashMap::new();
        // let mut map: Map<String, Value> = Map::new();
        // map.insert(String::from("content"), Value::from(self.package_text(sender, content)));
        params.insert(String::from("text"), Value::from(self.package_text(sender, content)));
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        println!("POST {} {:?} {}", &self.slackbot, headers, &serde_json::to_string(&params).unwrap());
        self.http_client.send_request("POST", &self.slackbot, headers, &serde_json::to_string(&params).unwrap()).await;
    }
}