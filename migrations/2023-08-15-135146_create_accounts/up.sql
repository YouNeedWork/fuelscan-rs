-- Your SQL goes here
create table
  accounts (
    id character varying(255) not null,
    timestamp integer null,
    constraint accounts_pkey primary key (id)
  ) tablespace pg_default;