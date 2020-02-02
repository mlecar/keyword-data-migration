pub mod database_statistics;
pub mod http_pool;
pub mod json_to_model_mapper;
pub mod keywords;
pub mod response_filter;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct KeywordResult {
    request: Value,
    response: Value,
}
