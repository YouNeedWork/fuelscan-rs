-- Your SQL goes here
create table
  contracts (
    contract_id character varying(255) not null,
    transaction_id character varying(255) not null,
    sender character varying(255) not null,
    bytecode text not null,
    bytecoin_length BIGINT not null,
    storage_slots json null,

    timestamp BIGINT not null,
    constraint contracts_pkey primary key (contract_id)
  ) tablespace pg_default;