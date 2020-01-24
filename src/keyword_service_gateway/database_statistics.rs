use crate::keyword_service_gateway::KeywordResult;
use log::warn;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use std::convert::TryFrom;
use std::error::Error;

pub fn get_statistics(client: &Client, statistics_url: &String) -> Result<i64, Box<dyn Error>> {
    let resp_statistics = client.get(statistics_url).send()?;
    match resp_statistics.status() {
        StatusCode::OK => {
            let v: KeywordResult = resp_statistics.json()?;
            let unused_count = v
                .response
                .as_object()
                .unwrap()
                .get("keyword.db.keyword_id.unused_count")
                .unwrap()
                .as_i64()
                .unwrap();
            Ok(unused_count)
        }
        s => {
            warn!(
                "Received response status: {:?}, body {:?}",
                s,
                resp_statistics.text()
            );
            Err(Box::try_from("Failed to get statistics").unwrap())
        }
    }
}
