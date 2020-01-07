create table if not exists keyword_test (
                              keyword_id bigserial primary key,
                              keyword varchar(200) not null,
                              keyword_filtered varchar(200),
                              created_at timestamp not null DEFAULT NOW()
)