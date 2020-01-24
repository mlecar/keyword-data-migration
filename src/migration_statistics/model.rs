use super::schema::migration_statistics;

#[derive(Insertable)]
#[table_name = "migration_statistics"]
pub struct MigrationStatistics<'a> {
    pub id: Option<i64>,
    pub unused_count: i64,
    pub migrated_from_id: i64,
    pub migrated_to_id: i64,
    pub migration_step: &'a str,
}
