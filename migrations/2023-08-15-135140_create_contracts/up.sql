-- Your SQL goes here
create table
  contracts (
    contract_id  varchar not null,
    transaction_id  varchar not null,
    sender  varchar not null,
    bytecode text not null,
    bytecoin_length BIGINT not null,
    storage_slots json null,

    timestamp BIGINT not null,
    constraint contracts_pkey primary key (contract_id)
  ) tablespace pg_default;