-- Your SQL goes here

create table users (
    id serial primary key,
    username text not null,
    email text not null ,
    password_hash text not null,
    created_at  timestamp default CURRENT_TIMESTAMP not null,
    updated_at  timestamp default CURRENT_TIMESTAMP not null
);

SELECT diesel_manage_updated_at('users');