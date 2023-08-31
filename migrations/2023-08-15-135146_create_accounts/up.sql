-- Your SQL goes here
create table
  accounts (
    id bigint,
    account_hash varchar not null,
    account_code text null,
    account_name text null,
    account_type text null,-- contract or account?
    verified bool not null,
    gas_used int8 not null,
    transactions_count not null,
    token_transfers_count not null,        
    sender_count not null,
    recever_count not null,
    decompiled bool not null,
    inserted_at timestamp not null,
    updated_at timestamp not null,
    constraint accounts_pkey primary key (id)
  ) tablespace pg_default;