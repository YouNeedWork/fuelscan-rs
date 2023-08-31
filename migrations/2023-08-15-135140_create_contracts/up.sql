-- Your SQL goes here
create table
  smart_contracts (
    contract_hash  varchar not null,
    transaction_id  varchar not null,
    sender  varchar not null,
    bytecode text not null,
    bytecoin_length BIGINT not null,
    storage_slots json null,

    timestamp BIGINT not null,
    constraint contracts_pkey primary key (contract_hash)
  ) tablespace pg_default;