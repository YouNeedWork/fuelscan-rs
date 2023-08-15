-- Your SQL goes here
create table
  coinbases (
    id character varying(255) not null,
    height integer null,
    da_height integer null,
    block_hash character varying(255) not null,
    amount character varying(255) null,
    coinbase character varying(255) null,
    timestamp integer null,
    constraint coinbases_pkey primary key (id)
  ) tablespace pg_default;