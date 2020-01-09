create table if not exists keyword_test (
                              keyword_id bigint PRIMARY KEY AUTO_INCREMENT,
                              keyword nvarchar(200) not null,
                              created_at timestamp not null DEFAULT NOW()
)

create table if not exists unused_keyword_id (
                                            keyword_id bigint PRIMARY KEY AUTO_INCREMENT,
                                            created_at timestamp not null DEFAULT NOW()
)