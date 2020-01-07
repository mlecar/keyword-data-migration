extern crate config;

#[macro_use]
extern crate serde_json;
extern crate serde;

use reqwest::StatusCode;
use config::Config;
use serde_json::Value;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct KeywordResult {
    request: Value,
    response: Value
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read configurations
    let mut settings: Config = config::Config::default();
    settings.merge(config::File::with_name("settings")).unwrap();

    let keyword_url:String = settings.get::<String>("keywordurl").unwrap();
    println!("Settings {:?}", &keyword_url);

    // prepare params
    let mut keyword_vec: Vec<i64> = Vec::new();
    for x in 0..10 {
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
            for x in v.response.as_array().unwrap() {
                println!("{:?}, {:?}", x["keyword"].as_str().unwrap(), x["keyword_id"].as_i64().unwrap());
            }
        },
        s => {
            println!("Received response status: {:?}, body {:?}", s, resp.text());
        },
    };

    Ok(())
}