use crate::keyword_service_gateway::http_pool::HttpPool;
use crate::keyword_service_gateway::response_filter::remove_invalid_keywords;
use crate::keyword_service_gateway::KeywordResult;
use crate::keywords::repository::save_keywords_batch;
use crate::unused_keywords::repository::save_unused_keywords_batch;
use diesel::MysqlConnection;
use log::warn;
use reqwest::StatusCode;
use std::convert::TryFrom;
use std::error::Error;

pub fn get_keywords(http_pool: &HttpPool, keyword_vec: &Vec<i64>, keyword_url: &str, conn: &MysqlConnection) -> Result<(), Box<dyn Error>> {
    let item = json!({
        "keyword_id": keyword_vec,
        "ignore_errors": "true"
    });

    match http_pool.get_connection().get(keyword_url).json(&item).send() {
        Err(e) => return Err(Box::try_from(format!("Request get keywords failed {:?}", e)).unwrap()),
        Ok(response) => match response.status() {
            StatusCode::OK => {
                let v: KeywordResult = response.json()?;
                let (valid_keywords, valid_unused_keywords) = remove_invalid_keywords(&v);
                save_keywords_batch(conn, &valid_keywords)?;
                save_unused_keywords_batch(conn, &valid_unused_keywords)?;
            }
            s => {
                warn!("Received response status: {:?}, body {:?}", s, response.text());
                return Err(Box::try_from(format!("Request get keywords failed {}", s)).unwrap());
            }
        },
    };
    Ok(())
}
