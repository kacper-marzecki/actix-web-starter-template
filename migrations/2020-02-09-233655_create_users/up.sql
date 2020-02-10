-- Your SQL goes here

create table users (
    id serial primary key,
    username text not null,
    email text not null ,
    password_hash text not null,
    created_at  timestamp default CURRENT_TIMESTAMP not null,
    updated_at  timestamp default CURRENT_TIMESTAMP not null
);

create unique index users_unique_username on users (username);
create unique index users_unique_email on users (email);

SELECT diesel_manage_updated_at('users');