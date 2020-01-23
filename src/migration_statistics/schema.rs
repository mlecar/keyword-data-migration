table! {
    migration_statistics (id) {
        id -> BigInt,
        unused_count -> BigInt,
        migrated_from_id -> BigInt,
        migrated_to_id -> BigInt,
        migration_step -> Text,
    }
}
