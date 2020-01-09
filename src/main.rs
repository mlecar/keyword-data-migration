extern crate config;

#[macro_use]
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate mysql;
extern crate simple_logger;

use reqwest::StatusCode;
use config::Config;
use serde_json::Value;
use serde::{Deserialize};
use log::{info, warn, Level};
use mysql::Pool;

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

    // client for request
    let client = reqwest::blocking::Client::new();

    // flow
    let mut start = keyword_id_start;
    let mut end = keyword_id_start + 500;

    // loop
    while end <= current_max_keyword_id {
        // prepare params
        let mut keyword_vec: Vec<i64> = Vec::new();
        for x in start..end {
            keyword_vec.push(x);
            //info!("{:?}", x);
        }

        let item = json!({
            "keyword_id": &keyword_vec,
            "ignore_errors": true
        });

        let resp = client.get(&keyword_url).json(&item).send()?;

        let pool = mysql::Pool::new("mysql://root:sa@localhost:3306/keyword_test").unwrap();

        match resp.status() {
            StatusCode::OK => {
                let v: KeywordResult = resp.json()?;

                let valid_keywords = v.response.as_array().unwrap().iter().filter(|&k| k["keyword"].as_str().unwrap().trim() != "" ).collect::<Vec<_>>();
                let empty_keywords = v.response.as_array().unwrap().iter().filter(|&k| k["keyword"].as_str().unwrap().trim() == "" ).collect::<Vec<_>>();

                info!("valid_keywords {:?}", valid_keywords);
                info!("empty_keywords {:?}", empty_keywords);

                save_keyword(&valid_keywords, &pool);
                save_unused_keyword_ids(&empty_keywords, &pool);
            },
            s => {
                warn!("Received response status: {:?}, body {:?}", s, resp.text());
            },
        };
        info!("Imported keywords from {:?} to {:?}", start, end);
        start+=500;
        end = start + 500;
    }

    Ok(())
}

fn save_keyword(valid_keywords: &Vec<&Value>, pool:&Pool) {
    // stringprefix r means it will be treated as a raw string
    for mut stmt in pool.prepare(r"INSERT IGNORE INTO keyword_test (id, keyword) VALUES (:keyword_id, :keyword)").into_iter() {
        for x in valid_keywords {
            let keyword_str = x["keyword"].as_str().unwrap();
            let keyword_id = x["keyword_id"].as_i64().unwrap();
            stmt.execute(params! {
                            "keyword_id" => keyword_id,
                            "keyword" => keyword_str
                        }).unwrap();
        }
    }
}

fn save_unused_keyword_ids(empty_keywords: &Vec<&Value>, pool:&Pool) {
    // stringprefix r means it will be treated as a raw string
    for mut stmt in pool.prepare(r"INSERT IGNORE INTO unused_keyword_id_test (id) VALUES (:keyword_id)").into_iter() {
        for x in empty_keywords {
            let keyword_id = x["keyword_id"].as_i64().unwrap();
            stmt.execute(params! {
                            "keyword_id" => keyword_id,
                        }).unwrap();
        }
    }
}