extern crate diesel;
extern crate keyword_data_migration;

use keyword_data_migration::database_connection::establish_connection;
use keyword_data_migration::keywords::repository::save_keyword;

fn main() {
    let connection = establish_connection();

    match save_keyword(&connection, 6000 as i64, "dvd neuerscheinungen") {
        Err(e) => println!("{:?}", e),
        _ => (),
    }
}
