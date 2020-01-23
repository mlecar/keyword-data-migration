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

use keyword_data_migration::database_connection::establish_connection;
use keyword_data_migration::json_to_model_mapper::{map_keywords, map_unused_keywords};
use keyword_data_migration::keyword_service_gateway::database_statistics::get_statistics;
use keyword_data_migration::keyword_service_gateway::response_filter::remove_invalid_keywords;
use keyword_data_migration::keyword_service_gateway::KeywordResult;
use keyword_data_migration::keywords::repository::save_keywords_batch;
use keyword_data_migration::migration_statistics::repository::save_migration_statistic;
use keyword_data_migration::unused_keywords::repository::save_unused_keywords_batch;

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
    let increment: i64 = settings.get::<i64>("increment").unwrap();
    let last_inserted_record: i64 = current_max_keyword_id - 1;

    // database connection
    let conn = establish_connection();

    // client for request
    let client = Client::new();

    let unused_count = get_statistics(&client, &statistics_url)?;
    save_migration_statistic(
        &conn,
        &unused_count,
        &keyword_id_start,
        &last_inserted_record,
        "START",
    )?;

    // flow
    let mut start = keyword_id_start;
    let mut end = keyword_id_start;

    // loop
    while end < current_max_keyword_id {
        let exec_time = Instant::now();
        end += increment;

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
                save_migration_statistic(
                    &conn,
                    &unused_count,
                    &keyword_id_start,
                    &last_inserted_record,
                    format!("Error {:?} ", e).as_ref(),
                )?;
            }
            Ok(response) => {
                match response.status() {
                    StatusCode::OK => {
                        let v: KeywordResult = response.json()?;
                        let (valid_keywords, valid_unused_keywords) = remove_invalid_keywords(&v);
                        let keywords_to_store = map_keywords(valid_keywords);
                        let unused_keywords_to_store = map_unused_keywords(valid_unused_keywords);
                        save_keywords_batch(&conn, &keywords_to_store)?;
                        save_unused_keywords_batch(&conn, &unused_keywords_to_store)?;
                    }
                    s => {
                        warn!(
                            "Received response status: {:?}, body {:?}",
                            s,
                            response.text()
                        );
                    }
                };
            }
        };

        info!(
            "Imported keywords from {:?} to {:?} in {:?} milliseconds. Total execution in {:?}",
            start,
            end - 1,
            exec_time.elapsed().as_millis(),
            now.elapsed().as_secs()
        );
        start = end;
    }
    save_migration_statistic(
        &conn,
        &unused_count,
        &keyword_id_start,
        &last_inserted_record,
        "END",
    )?;
    info!(
        "Total execution from {:?} to {:?} in {:?} seconds",
        keyword_id_start,
        current_max_keyword_id - 1,
        now.elapsed().as_secs()
    );
    Ok(())
}
