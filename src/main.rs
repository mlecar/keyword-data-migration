extern crate config;

#[macro_use]
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate mysql;

use reqwest::StatusCode;
use config::Config;
use serde_json::Value;
use serde::{Deserialize};
use mysql as my;

#[derive(Debug, Deserialize)]
struct KeywordResult {
    request: Value,
    response: Value
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read configurations
    let mut settings: Config = config::Config::default();
    settings.merge(config::File::with_name("settings")).unwrap();

    let keyword_url:String = settings.get::<String>("keyword_url").unwrap();
    let keyword_id_start = settings.get::<i64>("keyword_id_start").unwrap();
    let current_max_keyword_id = settings.get::<i64>("max_keyword_id").unwrap();
    println!("Settings {:?}", &keyword_url);

    // prepare params
    let mut keyword_vec: Vec<i64> = Vec::new();
    for x in 0..100 {
        keyword_vec.push(x);
    }

    let item = json!({
        "keyword_id": &keyword_vec,
        "ignore_errors": true
    });

    // make request
    let client = reqwest::blocking::Client::new();
    let resp = client.get(&keyword_url).json(&item).send()?;

    match resp.status() {
        StatusCode::OK => {
            let v: KeywordResult = resp.json()?;

            let pool = my::Pool::new("mysql://root:sa@localhost:3306/keyword_test").unwrap();
            for mut stmt in pool.prepare(r"INSERT IGNORE INTO keyword_test (keyword_id, keyword) VALUES (:keyword_id, :keyword)").into_iter() {
                for x in v.response.as_array().unwrap() {

                    let keyword_str = x["keyword"].as_str().unwrap();
                    let keyword_id = x["keyword_id"].as_i64().unwrap();
                    if keyword_str.trim() == "" {
                        continue;
                    }
                    println!("{:?}, {:?}", x["keyword"].as_str().unwrap(), x["keyword_id"].as_i64().unwrap());

                    stmt.execute(params!{
                        "keyword_id" => keyword_id,
                        "keyword" => keyword_str
                    }).unwrap();
                }
            }
        },
        s => {
            println!("Received response status: {:?}, body {:?}", s, resp.text());
        },
    };

    Ok(())
}