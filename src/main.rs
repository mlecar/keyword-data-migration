extern crate config;
extern crate keyword_data_migration;
extern crate serde;
extern crate simple_logger;

use std::time::Instant;

use config::Config;
use log::{info, Level};

use keyword_data_migration::database_connection::establish_connection;
use keyword_data_migration::keyword_service_gateway::database_statistics::get_statistics;
use keyword_data_migration::keyword_service_gateway::http_pool::HttpPool;
use keyword_data_migration::keyword_service_gateway::keywords::get_keywords;
use keyword_data_migration::migration_statistics::repository::save_migration_statistic;

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

    let http_pool = HttpPool::new();

    let unused_count = get_statistics(&http_pool, &statistics_url)?;
    save_migration_statistic(&conn, &unused_count, &keyword_id_start, &current_max_keyword_id, "START")?;

    let mut keywords_list: Vec<i64> = (keyword_id_start..current_max_keyword_id).collect();

    keywords_list.chunks_mut(batch_size as usize).for_each(| keywords | {
        let exec_time = Instant::now();
        match get_keywords(&http_pool, &keywords.to_vec(), &keyword_url, &conn) {
            Ok(()) => {
                info!(
                    "Imported keywords from {:?} to {:?} in {:?} milliseconds. Total execution in {:?}",
                    keywords.first().unwrap(),
                    keywords.last().unwrap(),
                    exec_time.elapsed().as_millis(),
                    now.elapsed().as_secs()
                );
            }
            _ => {
                save_migration_statistic(&conn, &unused_count, &keyword_id_start, &current_max_keyword_id, "ERROR ").unwrap();
            }
        }
    });

    save_migration_statistic(&conn, &unused_count, &keyword_id_start, &current_max_keyword_id, "END").unwrap();
    info!(
        "Total execution from {:?} to {:?} in {:?} seconds",
        keyword_id_start,
        current_max_keyword_id,
        now.elapsed().as_secs()
    );
    Ok(())
}
