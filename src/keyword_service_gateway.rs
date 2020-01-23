pub mod database_statistics;
pub mod response_filter;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct KeywordResult {
    request: Value,
    response: Value,
}
