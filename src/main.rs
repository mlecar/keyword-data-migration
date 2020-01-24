extern crate config;
extern crate keyword_data_migration;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate simple_logger;

use std::time::Instant;

use config::Config;
use log::{info, warn, Level};
use reqwest::blocking::Client;
use reqwest::StatusCode;

use diesel::MysqlConnection;
use keyword_data_migration::database_connection::establish_connection;
use keyword_data_migration::json_to_model_mapper::{map_keywords, map_unused_keywords};
use keyword_data_migration::keyword_service_gateway::database_statistics::get_statistics;
use keyword_data_migration::keyword_service_gateway::response_filter::remove_invalid_keywords;
use keyword_data_migration::keyword_service_gateway::KeywordResult;
use keyword_data_migration::keywords::repository::save_keywords_batch;
use keyword_data_migration::migration_statistics::repository::save_migration_statistic;
use keyword_data_migration::unused_keywords::repository::save_unused_keywords_batch;
use std::convert::TryFrom;
use std::error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();
    simple_logger::init_with_level(Level::Info).unwrap();
    info!("Starting import");

    // read configurations
    let mut settings: Config = config::Config::default();
    settings.merge(config::File::with_name("settings")).unwrap();

    let keyword_url: String = settings.get::<String>("keyword_url").unwrap();
    let keyword_id_start = settings.get::<i64>("keyword_id_start").unwrap();
    let current_max_keyword_id = settings.get::<i64>("max_keyword_id").unwrap();
    let statistics_url: String = settings.get::<String>("statistics_url").unwrap();
    let batch_size: u64 = settings.get::<u64>("batch_size").unwrap();

    // database connection
    let conn = establish_connection();

    // client for request
    let client = Client::new();

    let unused_count = get_statistics(&client, &statistics_url)?;
    save_migration_statistic(&conn, &unused_count, &keyword_id_start, &current_max_keyword_id, "START")?;

    let mut keyword_vec: Vec<i64> = Vec::new();
    for x in keyword_id_start..=current_max_keyword_id {
        keyword_vec.push(x);
        if x == current_max_keyword_id || keyword_vec.len() == batch_size as usize {
            // request service
            // store keyword
            // store unused
            let exec_time = Instant::now();
            match process(&keyword_vec, &client, &keyword_url, &conn) {
                Ok(()) => {
                    info!(
                        "Imported keywords from {:?} to {:?} in {:?} milliseconds. Total execution in {:?}",
                        keyword_vec.get(0).unwrap(),
                        keyword_vec.last().unwrap(),
                        exec_time.elapsed().as_millis(),
                        now.elapsed().as_secs()
                    );
                    keyword_vec.clear();
                }
                _ => {
                    save_migration_statistic(&conn, &unused_count, &keyword_id_start, &current_max_keyword_id, "ERROR ")?;
                }
            }
        }
    }
    save_migration_statistic(&conn, &unused_count, &keyword_id_start, &current_max_keyword_id, "END")?;
    info!(
        "Total execution from {:?} to {:?} in {:?} seconds",
        keyword_id_start,
        current_max_keyword_id,
        now.elapsed().as_secs()
    );
    Ok(())
}

fn process(keyword_vec: &Vec<i64>, client: &Client, keyword_url: &str, conn: &MysqlConnection) -> Result<(), Box<dyn Error>> {
    let item = json!({
        "keyword_id": keyword_vec,
        "ignore_errors": true
    });

    match client.get(keyword_url).json(&item).send() {
        Err(e) => return Err(Box::try_from(format!("Request get keywords failed {:?}", e)).unwrap()),
        Ok(response) => match response.status() {
            StatusCode::OK => {
                let v: KeywordResult = response.json()?;
                let (valid_keywords, valid_unused_keywords) = remove_invalid_keywords(&v);
                let keywords_to_store = map_keywords(valid_keywords);
                let unused_keywords_to_store = map_unused_keywords(valid_unused_keywords);
                save_keywords_batch(conn, &keywords_to_store)?;
                save_unused_keywords_batch(conn, &unused_keywords_to_store)?;
            }
            s => {
                warn!("Received response status: {:?}, body {:?}", s, response.text());
                return Err(Box::try_from(format!("Request get keywords failed {}", s)).unwrap());
            }
        },
    };
    Ok(())
}
