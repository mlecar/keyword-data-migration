extern crate keyword_data_migration;
extern crate diesel;

use self::keyword_data_migration::*;
use std::borrow::Borrow;

fn main() {
    let connection = establish_connection();

    match save_keyword(&connection, (6000 as i64).borrow(), "dvd neuerscheinungen") {
        Err(e) => println!("{:?}", e),
        _ => ()
    }
}