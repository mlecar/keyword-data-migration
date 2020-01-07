extern crate config;

#[macro_use]
extern crate serde_json;
extern crate serde;
extern crate postgres;

use reqwest::StatusCode;
use config::Config;
use serde_json::Value;
use serde::{Deserialize};
use postgres::{Client, NoTls};

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

            let mut client = Client::connect("host=localhost user=marcelolecar dbname=frybot_test", NoTls)?;

            for x in v.response.as_array().unwrap() {
                let keyword_str = x["keyword"].as_str().unwrap();
                let keyword_id = x["keyword_id"].as_i64().unwrap();
                if keyword_str.trim() == "" {
                    continue;
                }
                client.execute("INSERT INTO keyword_test (keyword_id, keyword) VALUES ($1, $2) ON CONFLICT DO NOTHING", &[&keyword_id, &keyword_str])?;
                println!("{:?}, {:?}", x["keyword"].as_str().unwrap(), x["keyword_id"].as_i64().unwrap());
            }
        },
        s => {
            println!("Received response status: {:?}, body {:?}", s, resp.text());
        },
    };

    Ok(())
}