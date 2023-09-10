DO
$$
BEGIN
CREATE TYPE account_type AS ENUM ('account', 'contract');
END
$$;

-- Your SQL goes here
create table
  accounts (
    account_hash varchar not null,
    account_code text null,
    account_name text null,
    account_type account_type not null,-- contract or account?
    verified bool not null,
    gas_used int8 not null,
    transactions_count BIGSERIAL not null,
    token_transfers_count BIGSERIAL not null,        
    sender_count BIGSERIAL not null,
    recever_count BIGSERIAL not null,
    decompiled bool not null,
    inserted_at timestamp not null,
    updated_at timestamp not null,
    constraint accounts_pkey primary key (account_hash)
  ) tablespace pg_default;
