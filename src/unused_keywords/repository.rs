use crate::unused_keywords::model::UnusedKeywordId;
use crate::unused_keywords::schema::unused_keyword_id;
use diesel::result::Error;
use diesel::{MysqlConnection, RunQueryDsl};

pub fn save_unused_keywords_batch<'a>(
    conn: &MysqlConnection,
    unused_keyword_ids: &Vec<UnusedKeywordId>,
) -> Result<(), Error> {
    diesel::insert_or_ignore_into(unused_keyword_id::table)
        .values(unused_keyword_ids)
        .execute(conn)
        .expect("Error saving new unused");
    Ok(())
}
