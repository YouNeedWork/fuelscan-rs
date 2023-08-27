-- Your SQL goes here
DO
$$
BEGIN
CREATE TYPE call_type AS ENUM ('contract', 'transaction');
END
$$;

create table
  calls (
    transaction_id  varchar not null,
    height BIGINT not null,
    da_height BIGINT not null,
    block_hash  varchar not null,
    call_type call_type not null,
    gas_limit BIGINT not null,
    gas_price BIGINT not null,
    gas_used BIGINT not null,
    
    sender  varchar not null,
    receiver  varchar not null,

    amount BIGINT null,
    asset_id  varchar null,

    payload text null,
    payload_data text null,

    timestamp BIGINT null,
    constraint calls_pkey primary key (transaction_id)
  ) tablespace pg_default;