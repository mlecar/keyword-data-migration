extern crate keyword_data_migration;
extern crate config;

#[macro_use]
extern crate serde_json;
extern crate serde;
extern crate simple_logger;

use reqwest::StatusCode;
use config::Config;
use serde_json::Value;
use serde::{Deserialize};
use log::{info, warn, Level};
use self::keyword_data_migration::*;
use keyword_data_migration::models::{Keyword, UnusedKeywordId};

#[derive(Debug, Deserialize)]
struct KeywordResult {
    request: Value,
    response: Value
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(Level::Info).unwrap();
    info!("Starting import");

    // read configurations
    let mut settings: Config = config::Config::default();
    settings.merge(config::File::with_name("settings")).unwrap();

    let keyword_url:String = settings.get::<String>("keyword_url").unwrap();
    let keyword_id_start = settings.get::<i64>("keyword_id_start").unwrap();
    let current_max_keyword_id = settings.get::<i64>("max_keyword_id").unwrap();

    let increment = 2000;

    // client for request
    let client = reqwest::blocking::Client::new();

    // flow
    let mut start = keyword_id_start;
    let mut end = keyword_id_start;

    let conn = establish_connection();

    // loop
    while end < current_max_keyword_id {
        end+=increment;

        // prepare params
        let mut keyword_vec: Vec<i64> = Vec::new();
        for x in start..end {
            keyword_vec.push(x);
            info!("{:?}", x);
        }

        let item = json!({
            "keyword_id": &keyword_vec,
            "ignore_errors": true
        });

        let resp = client.get(&keyword_url).json(&item).send()?;

        match resp.status() {
            StatusCode::OK => {
                let v: KeywordResult = resp.json()?;

                let valid_keywords = v.response.as_array().unwrap().iter().filter(|&k| k["keyword"].as_str().unwrap().trim() != "" ).collect::<Vec<_>>();
                let empty_keywords = v.response.as_array().unwrap().iter().filter(|&k| k["keyword"].as_str().unwrap().trim() == "" ).collect::<Vec<_>>();

                info!("valid_keywords {:?}", valid_keywords);
                info!("empty_keywords {:?}", empty_keywords);

                let mut valid_keywords_2: Vec<Keyword> = Vec::new();
                for x in valid_keywords {
                    let keyword_str = x["keyword"].as_str().unwrap();
                    let keyword_id = x["keyword_id"].as_i64().unwrap();
                    valid_keywords_2.push(Keyword {
                        keyword_str,
                        id: &keyword_id,
                    });
                }

                let mut empty_keywords_2: Vec<UnusedKeywordId> = Vec::new();
                for x in empty_keywords {
                    let keyword_id = x["keyword_id"].as_i64().unwrap();
                    empty_keywords_2.push(UnusedKeywordId {
                        id: &keyword_id,
                    });
                }

                save_keywords_batch(&conn, &valid_keywords_2)?;
                save_unused_keywords_batch(&conn, &empty_keywords_2)?;
            },
            s => {
                warn!("Received response status: {:?}, body {:?}", s, resp.text());
            },
        };
        info!("Imported keywords from {:?} to {:?}", start, end-1);
        start = end;
    }
    Ok(())
}