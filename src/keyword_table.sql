create table if not exists keyword_test (
                              keyword_id bigint PRIMARY KEY AUTO_INCREMENT,
                              keyword nvarchar(200) not null,
                              keyword_filtered varchar(200),
                              created_at timestamp not null DEFAULT NOW()
)