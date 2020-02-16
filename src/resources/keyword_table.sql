
CREATE TABLE IF NOT EXISTS `keyword` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT 'PK',
  `keyword` varchar(200) COLLATE utf8_bin NOT NULL COMMENT 'keyword',
  PRIMARY KEY (`id`),
  UNIQUE KEY `udx_keyword` (`keyword`) USING BTREE
) ENGINE=InnoDB  DEFAULT CHARSET=utf8 COLLATE=utf8_bin AUTO_INCREMENT=1 ;

CREATE INDEX idx_keyword_id ON keyword (id);

CREATE TABLE `unused_keyword_id_test` (
    `id` bigint unsigned PRIMARY KEY COMMENT 'PK'
) ENGINE=MYISAM DEFAULT CHARSET=utf8 COLLATE=utf8_bin COMMENT='global keyword base for unused keyword id';

create table migration_statistics (
id bigint AUTO_INCREMENT PRIMARY KEY,
 unused_count bigint,
 migrated_from_id bigint,
 migrated_to_id bigint,
 migrated_at timestamp default now(),
 migration_step varchar(10)
)

ALTER TABLE migration_statistics ADD migration_step varchar(10)

alter table migration_statistics modify migration_step varchar(4000);

-- useful queries
select * from keyword order by id desc

select * from unused_keyword_id uki order by id desc

select * from migration_statistics ORDER by id desc

select * from keyword k where id < 4999 order by id desc

insert into keyword (id, keyword) select id, keyword from keyword_2 k

select count(id) from keyword k order by id desc

optimize table keyword

analyze table keyword



