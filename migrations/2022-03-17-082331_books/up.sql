-- Your SQL goes here
-- Add migration script here

create table if not exists books (
    id serial primary key,
    title text not null,
    description text not null unique,
    price integer not null default 0,
    rating integer not null default 0
);