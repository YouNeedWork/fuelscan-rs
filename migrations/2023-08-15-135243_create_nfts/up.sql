-- Your SQL goes here
create table
  nfts (
    id  varchar not null,
    timestamp BIGINT null,
    constraint nfts_pkey primary key (id)
  ) tablespace pg_default;