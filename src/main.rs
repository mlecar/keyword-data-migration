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
use log::{info, warn, error, Level};
use mysql::{Pool, Error};

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

    let database_host = settings.get::<String>("database_host").unwrap();
    let database_user = settings.get::<String>("database_user").unwrap();
    let database_password = settings.get::<String>("database_password").unwrap();
    let database_name = settings.get::<String>("database_name").unwrap();

    let increment = 2000;

    // client for request
    let client = reqwest::blocking::Client::new();

    // flow
    let mut start = keyword_id_start;
    let mut end = keyword_id_start;

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

        let database_connection_string = format!("mysql://{}:{}@{}:3306/{}", &database_user, &database_password, &database_host, &database_name);

        let pool = mysql::Pool::new(&database_connection_string).unwrap();

        match resp.status() {
            StatusCode::OK => {
                let v: KeywordResult = resp.json()?;

                let valid_keywords = v.response.as_array().unwrap().iter().filter(|&k| k["keyword"].as_str().unwrap().trim() != "" ).collect::<Vec<_>>();
                let empty_keywords = v.response.as_array().unwrap().iter().filter(|&k| k["keyword"].as_str().unwrap().trim() == "" ).collect::<Vec<_>>();

                info!("valid_keywords {:?}", valid_keywords);
                info!("empty_keywords {:?}", empty_keywords);

                if let Err(_err) = save_keyword(&valid_keywords, &pool) {
                    error!("Could not save keywords {:?}", _err);
                }
                if let Err(_err) = save_unused_keyword_ids(&empty_keywords, &pool){
                    error!("Could not save unused keywords {:?}", _err);
                }
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

fn save_keyword(valid_keywords: &Vec<&Value>, pool:&Pool) -> Result<(), Error> {
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
    Ok(())
}

fn save_unused_keyword_ids(empty_keywords: &Vec<&Value>, pool:&Pool) -> Result<(), Error>{
    // stringprefix r means it will be treated as a raw string
    for mut stmt in pool.prepare(r"INSERT IGNORE INTO unused_keyword_id_test (id) VALUES (:keyword_id)").into_iter() {
        for x in empty_keywords {
            let keyword_id = x["keyword_id"].as_i64().unwrap();
            stmt.execute(params! {"keyword_id" => keyword_id}).unwrap();
        }
    }
    Ok(())
}