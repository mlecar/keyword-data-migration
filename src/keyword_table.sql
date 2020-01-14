CREATE TABLE `keyword_test` (
                                `id` bigint unsigned PRIMARY KEY AUTO_INCREMENT COMMENT 'PK',
                                `keyword` nvarchar(200) NOT NULL COMMENT 'keyword',
    UNIQUE KEY `udx_keyword` (`keyword`) USING BTREE
    ) ENGINE=InnoDB AUTO_INCREMENT=3232148376 DEFAULT CHARSET=utf8 COLLATE=utf8_bin COMMENT='global keyword base';
/*!40101 SET character_set_client = @saved_cs_client */;

CREATE TABLE `unused_keyword_id_test` (
    `id` bigint unsigned PRIMARY KEY COMMENT 'PK'
) ENGINE=MYISAM DEFAULT CHARSET=utf8 COLLATE=utf8_bin COMMENT='global keyword base for unused keyword id';

create table migration_statistics (
id bigint AUTO_INCREMENT PRIMARY KEY,
 unused_count bigint,
 migrated_from_id bigint,
 migrated_to_id bigint,
 migrated_at timestamp default now()
)