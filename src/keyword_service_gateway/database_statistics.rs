use crate::keyword_service_gateway::http_pool::HttpPool;
use crate::keyword_service_gateway::KeywordResult;
use log::warn;
use reqwest::StatusCode;
use std::convert::TryFrom;
use std::error::Error;

pub fn get_statistics(http_pool: &HttpPool, statistics_url: &String) -> Result<i64, Box<dyn Error>> {
    let resp_statistics = http_pool.get_connection().get(statistics_url).send()?;
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
            warn!("Received response status: {:?}, body {:?}", s, resp_statistics.text());
            Err(Box::try_from("Failed to get statistics").unwrap())
        }
    }
}
