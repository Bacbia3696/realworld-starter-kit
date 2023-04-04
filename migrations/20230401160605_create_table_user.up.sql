-- Add migration script here
create table
  if not exists "user" (
    id serial primary key,
    username varchar not null default '' unique,
    email varchar not null default '' unique,
    password varchar not null default '',
    bio varchar not null default '',
    image varchar not null default '',
    created_at timestamptz not null default current_timestamp,
    updated_at timestamptz not null default current_timestamp
  );
