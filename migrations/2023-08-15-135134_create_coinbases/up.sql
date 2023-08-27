-- Your SQL goes here
create table
  coinbases (
    id  varchar not null,
    height BIGINT null,
    da_height BIGINT null,
    block_hash  varchar not null,
    amount BIGINT null,
    coinbase  varchar null,
    timestamp BIGINT,
    constraint coinbases_pkey primary key (id)
  ) tablespace pg_default;