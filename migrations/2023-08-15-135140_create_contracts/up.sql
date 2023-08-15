-- Your SQL goes here
create table
  contracts (
    id character varying(255) not null,
    timestamp integer null,
    constraint contracts_pkey primary key (id)
  ) tablespace pg_default;