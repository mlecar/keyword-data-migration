use super::schema::keyword;
use super::schema::unused_keyword_id;
use super::schema::migration_statistics;


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

#[derive(Insertable)]
#[table_name="migration_statistics"]
pub struct MigrationStatistics {
    pub id: Option<i64>,
    pub unused_count: i64,
    pub migrated_from_id: i64,
    pub migrated_to_id: i64,
}