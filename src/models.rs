use super::schema::keyword;
use super::schema::unused_keyword_id;


#[derive(Insertable)]
#[table_name="keyword"]
pub struct Keyword<'a> {
    pub id: i64,
    pub keyword_str: &'a str,
}

#[derive(Insertable)]
#[table_name="unused_keyword_id"]
pub struct UnusedKeywordId {
    pub id: i64,
}