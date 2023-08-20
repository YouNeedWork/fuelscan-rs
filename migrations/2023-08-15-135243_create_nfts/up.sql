-- Your SQL goes here
create table
  nfts (
    id character varying(255) not null,
    timestamp BIGINT null,
    constraint nfts_pkey primary key (id)
  ) tablespace pg_default;