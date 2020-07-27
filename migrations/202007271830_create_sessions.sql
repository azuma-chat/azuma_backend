create table if not exists sessions (
    id bigserial primary key,
    token text not null,
    userid bigint not null,
    expiration timestamp with time zone not null
)