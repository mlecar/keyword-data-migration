use crate::json_to_model_mapper::{map_keywords, map_unused_keywords};
use crate::keyword_service_gateway::response_filter::remove_invalid_keywords;
use crate::keyword_service_gateway::KeywordResult;
use crate::keywords::repository::save_keywords_batch;
use crate::unused_keywords::repository::save_unused_keywords_batch;
use diesel::MysqlConnection;
use log::warn;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use std::convert::TryFrom;
use std::error::Error;

pub fn get_keywords(keyword_vec: &Vec<i64>, client: &Client, keyword_url: &str, conn: &MysqlConnection) -> Result<(), Box<dyn Error>> {
    let item = json!({
        "keyword_id": keyword_vec,
        "ignore_errors": true
    });

    match client.get(keyword_url).json(&item).send() {
        Err(e) => return Err(Box::try_from(format!("Request get keywords failed {:?}", e)).unwrap()),
        Ok(response) => match response.status() {
            StatusCode::OK => {
                let v: KeywordResult = response.json()?;
                let (valid_keywords, valid_unused_keywords) = remove_invalid_keywords(&v);
                let keywords_to_store = map_keywords(valid_keywords);
                let unused_keywords_to_store = map_unused_keywords(valid_unused_keywords);
                save_keywords_batch(conn, &keywords_to_store)?;
                save_unused_keywords_batch(conn, &unused_keywords_to_store)?;
            }
            s => {
                warn!("Received response status: {:?}, body {:?}", s, response.text());
                return Err(Box::try_from(format!("Request get keywords failed {}", s)).unwrap());
            }
        },
    };
    Ok(())
}
