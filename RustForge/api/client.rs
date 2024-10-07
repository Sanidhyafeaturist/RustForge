use reqwest::blocking::Client;
use reqwest::Result;

pub struct APIClient {
    client: Client,
}

impl APIClient {
    pub fn new() -> Self {
        APIClient {
            client: Client::new(),
        }
    }

    pub fn fetch_data(&self, url: &str) -> Result<String> {
        let response = self.client.get(url).send()?;
        response.text()
    }
}
