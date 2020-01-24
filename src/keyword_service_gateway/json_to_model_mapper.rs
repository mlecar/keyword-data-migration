use crate::keywords::model::Keyword;
use crate::unused_keywords::model::UnusedKeywordId;
use serde_json::Value;

pub fn map_keywords(keywords_json: Vec<&Value>) -> Vec<Keyword> {
    keywords_json
        .iter()
        .map(|x| Keyword {
            keyword_str: x["keyword"].as_str().unwrap(),
            id: x["keyword_id"].as_i64().unwrap(),
        })
        .collect()
}

pub fn map_unused_keywords(unused_keywords_json: Vec<&Value>) -> Vec<UnusedKeywordId> {
    unused_keywords_json
        .iter()
        .map(|&k| UnusedKeywordId {
            id: k["keyword_id"].as_i64().unwrap(),
        })
        .collect()
}
