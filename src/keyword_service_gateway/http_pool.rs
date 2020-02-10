use reqwest::blocking::Client;
use std::borrow::Borrow;
use std::time::Duration;

pub struct HttpPool {
    connection: Client,
}

impl HttpPool {
    pub fn new() -> HttpPool {
        let client = Client::builder().timeout(Duration::new(30, 0)).build();
        HttpPool { connection: client.unwrap() }
    }

    pub fn get_connection(&self) -> &Client {
        self.connection.borrow()
    }
}
