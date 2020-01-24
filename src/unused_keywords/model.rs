use super::schema::unused_keyword_id;

#[derive(Insertable)]
#[table_name = "unused_keyword_id"]
pub struct UnusedKeywordId {
    pub id: i64,
}
