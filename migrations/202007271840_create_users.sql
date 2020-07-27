create table if not exists users (
    id bigserial primary key,
    created timestamp with time zone not null default now(),
    name text not null,
    password text not null,
    icon text,
    status text
)