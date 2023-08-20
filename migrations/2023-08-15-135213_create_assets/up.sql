-- Your SQL goes here
create table
  assets (
    id character varying(255) not null,
    timestamp BIGINT null,
    constraint assets_pkey primary key (id)
  ) tablespace pg_default;