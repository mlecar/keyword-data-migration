use crate::keyword_service_gateway::KeywordResult;
use serde_json::Value;

pub fn remove_invalid_keywords(v: &KeywordResult) -> (Vec<&Value>, Vec<&Value>) {
    let valid_keywords = v
        .response
        .as_array()
        .unwrap()
        .iter()
        .filter(|&k| k["keyword"].as_str().unwrap().trim() != "")
        .collect::<Vec<_>>();

    let empty_keywords = v
        .response
        .as_array()
        .unwrap()
        .iter()
        .filter(|&k| k["keyword"].as_str().unwrap().trim() == "")
        .collect::<Vec<_>>();

    //info!("valid_keywords {:?}", valid_keywords);
    //info!("empty_keywords {:?}", empty_keywords);

    (valid_keywords, empty_keywords)
}
