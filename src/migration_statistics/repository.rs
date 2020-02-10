use crate::migration_statistics::schema::migration_statistics;
use crate::migration_statistics::schema::migration_statistics::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{MysqlConnection, RunQueryDsl};

pub fn save_migration_statistic<'a>(
    conn: &MysqlConnection,
    unused_count_param: i64,
    migrated_from_id_param: i64,
    migrated_to_id_param: i64,
    migration_step_param: &str,
) -> Result<(), Error> {
    let new_migration_statistic = (
        unused_count.eq(unused_count_param),
        migrated_from_id.eq(migrated_from_id_param),
        migrated_to_id.eq(migrated_to_id_param),
        migration_step.eq(migration_step_param),
    );

    diesel::insert_into(migration_statistics::table)
        .values(&new_migration_statistic)
        .execute(conn)
        .expect("Error saving new migration statistics");
    Ok(())
}
