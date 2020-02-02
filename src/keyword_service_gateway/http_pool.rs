use reqwest::blocking::Client;
use std::borrow::Borrow;

pub struct HttpPool {
    connection: Client,
}

impl HttpPool {
    pub fn new() -> HttpPool {
        HttpPool { connection: Client::new() }
    }

    pub fn get_connection(&self) -> &Client {
        self.connection.borrow()
    }
}
