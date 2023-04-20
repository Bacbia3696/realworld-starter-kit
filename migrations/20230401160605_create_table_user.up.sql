-- Add migration script here
create table
  if not exists "user" (
    id serial primary key,
    username varchar not null default '' unique,
    email varchar not null default '' unique,
    password varchar not null default '',
    bio varchar,
    image varchar,
    created_at timestamptz not null default current_timestamp,
    updated_at timestamptz not null default current_timestamp
  );
