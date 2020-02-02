use crate::keyword_service_gateway::KeywordResult;
use crate::keywords::model::Keyword;
use crate::unused_keywords::model::UnusedKeywordId;

pub fn remove_invalid_keywords(v: &KeywordResult) -> (Vec<Keyword>, Vec<UnusedKeywordId>) {
    let mut valid_keywords = Vec::new();
    let mut empty_keywords = Vec::new();

    for k in v.response.as_array().unwrap().iter() {
        if k["keyword"].as_str().unwrap().trim() != "" {
            valid_keywords.push(Keyword {
                keyword_str: k["keyword"].as_str().unwrap(),
                id: k["keyword_id"].as_i64().unwrap(),
            });
        } else {
            empty_keywords.push(UnusedKeywordId {
                id: k["keyword_id"].as_i64().unwrap(),
            });
        }
    }

    //info!("valid_keywords {:?}", valid_keywords);
    //info!("empty_keywords {:?}", empty_keywords);

    (valid_keywords, empty_keywords)
}
