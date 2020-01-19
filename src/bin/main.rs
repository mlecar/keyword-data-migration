extern crate config;
extern crate keyword_data_migration;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate simple_logger;

use std::convert::TryFrom;
use std::error::Error;
use std::time::Instant;

use config::Config;
use log::{info, Level, warn};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;

use keyword_data_migration::models::{Keyword, UnusedKeywordId};

use self::keyword_data_migration::*;
use diesel::MysqlConnection;

#[derive(Debug, Deserialize)]
struct KeywordResult {
    request: Value,
    response: Value
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();
    simple_logger::init_with_level(Level::Info).unwrap();
    info!("Starting import");

    // read configurations
    let mut settings: Config = config::Config::default();
    settings.merge(config::File::with_name("settings")).unwrap();

    let keyword_url:String = settings.get::<String>("keyword_url").unwrap();
    let keyword_id_start = settings.get::<i64>("keyword_id_start").unwrap();
    let current_max_keyword_id = settings.get::<i64>("max_keyword_id").unwrap();
    let statistics_url:String = settings.get::<String>("statistics_url").unwrap();
    let increment:i64 = settings.get::<i64>("increment").unwrap();
    let last_inserted_record:i64 = current_max_keyword_id-1;

    // database connection
    let conn = establish_connection();

    // client for request
    let client = Client::new();

    let unused_count = get_statistics(&client, &statistics_url)?;
    //info!("Unused ids count: {:?}",&unused_count);
    save_migration_statistic(&conn,
                             &unused_count,
                             &keyword_id_start,
                             &last_inserted_record,
                             "START")?;

    // flow
    let mut start = keyword_id_start;
    let mut end = keyword_id_start;

    // loop
    while end < current_max_keyword_id {
        let exec_time = Instant::now();
        end+=increment;

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

        match client.get(&keyword_url).json(&item).send() {
            Err(e) => {
                save_migration_statistic(&conn,
                                         &unused_count,
                                         &keyword_id_start,
                                         &last_inserted_record,
                                         format!("Error {:?} ", e).as_ref())?;
            },
            Ok(response) => {
                match response.status() {
                    StatusCode::OK => {
                        let v: KeywordResult = response.json()?;
                        read_response_and_save_database(&v, &conn)?;
                    },
                    s => {
                        warn!("Received response status: {:?}, body {:?}", s, response.text());
                    },
                };
            }
        };

        info!("Imported keywords from {:?} to {:?} in {:?} milliseconds. Total execution in {:?}", start, end-1, exec_time.elapsed().as_millis(), now.elapsed().as_secs());
        start = end;
    }
    save_migration_statistic(&conn,
                             &unused_count,
                             &keyword_id_start,
                             &last_inserted_record,
                              "END")?;
    info!("Total execution from {:?} to {:?} in {:?} seconds", keyword_id_start, current_max_keyword_id-1, now.elapsed().as_secs());
    Ok(())
}

fn get_statistics(client: &Client, statistics_url:&String) -> Result<i64, Box<dyn Error>>{
    let resp_statistics = client.get(statistics_url).send()?;
    match resp_statistics.status(){
        StatusCode::OK => {
            let v: KeywordResult = resp_statistics.json()?;
            let unused_count = v.response.as_object().unwrap().get("keyword.db.keyword_id.unused_count").unwrap().as_i64().unwrap();
            Ok(unused_count)
        },
        s => {
            warn!("Received response status: {:?}, body {:?}", s, resp_statistics.text());
            Err(Box::try_from("Failed to get statistics").unwrap())
        },
    }
}

fn read_response_and_save_database (v: &KeywordResult, conn:&MysqlConnection) -> Result<(), Box<dyn Error>> {
    let valid_keywords = v.response.as_array().unwrap().iter()
        .filter(|&k| k["keyword"].as_str().unwrap().trim() != "" ).collect::<Vec<_>>();

    let empty_keywords = v.response.as_array().unwrap().iter()
        .filter(|&k| k["keyword"].as_str().unwrap().trim() == "" ).collect::<Vec<_>>();

    //info!("valid_keywords {:?}", valid_keywords);
    //info!("empty_keywords {:?}", empty_keywords);

    let valid_keywords_2: Vec<Keyword> =
        valid_keywords.iter().map(|x| Keyword {
            keyword_str: x["keyword"].as_str().unwrap(),
            id: x["keyword_id"].as_i64().unwrap(),
        }).collect();

    let empty_keywords_2: Vec<UnusedKeywordId> =
        empty_keywords.iter().map(|&k| UnusedKeywordId { id: k["keyword_id"].as_i64().unwrap()}).collect();

    save_keywords_batch(conn, &valid_keywords_2)?;
    save_unused_keywords_batch(conn, &empty_keywords_2)?;
    Ok(())
}