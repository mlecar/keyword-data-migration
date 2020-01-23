use crate::keywords::model::Keyword;
use crate::keywords::schema::keyword;
use diesel::result::Error;
use diesel::{MysqlConnection, RunQueryDsl};

pub fn save_keyword<'a>(conn: &MysqlConnection, id: i64, keyword: &'a str) -> Result<(), Error> {
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

pub fn save_keywords_batch<'a>(
    conn: &MysqlConnection,
    keywords: &Vec<Keyword>,
) -> Result<(), Error> {
    diesel::insert_or_ignore_into(keyword::table)
        .values(keywords)
        .execute(conn)
        .expect("Error saving new keyword");
    Ok(())
}
