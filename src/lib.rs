pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_mysql;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use self::models::{Keyword, UnusedKeywordId};
use dotenv::dotenv;
use std::env;
use diesel::result::Error;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_host = env::var("DATABASE_HOST");
    let database_name = env::var("DATABASE_NAME");
    let database_user = env::var("DATABASE_USER");
    let database_password = env::var("DATABASE_PASSWORD");

    let database_connection_string = format!("mysql://{}:{}@{}:3306/{}",
                                             database_user.unwrap(), database_password.unwrap(),
                                             database_host.unwrap(), database_name.unwrap());

    MysqlConnection::establish(&database_connection_string)
        .expect(&format!("Error connecting to {}", &database_connection_string))
}

pub fn save_keyword<'a>(conn: &MysqlConnection, id: &'a i64, keyword: &'a str) -> Result<(), Error> {
    use schema::keyword;

    let new_keyword = Keyword {
        id,
        keyword_str: keyword,
    };

    diesel::insert_or_ignore_into(keyword::table)
        .values(&new_keyword)
        .execute(conn)
        .expect("Error saving new keyword");
    Ok(())
}

pub fn save_keywords_batch<'a>(conn: &MysqlConnection, keywords:&Vec<Keyword>) -> Result<(), Error> {
    use schema::keyword;

    diesel::insert_or_ignore_into(keyword::table)
        .values(keywords)
        .execute(conn)
        .expect("Error saving new keyword");
    Ok(())
}

pub fn save_unused_keywords_batch<'a>(conn: &MysqlConnection, unused_keyword_ids:&Vec<UnusedKeywordId>) -> Result<(), Error> {
    use schema::unused_keyword_id;

    diesel::insert_or_ignore_into(unused_keyword_id::table)
        .values(unused_keyword_ids)
        .execute(conn)
        .expect("Error saving new unused");
    Ok(())
}