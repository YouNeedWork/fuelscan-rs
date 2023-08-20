-- Your SQL goes here
DO
$$
BEGIN
CREATE TYPE call_type AS ENUM ('contract', 'transaction');
END
$$;

create table
  calls (
    transaction_id character varying(255) not null,
    height BIGINT not null,
    da_height BIGINT not null,
    block_hash character varying(255) not null,
    call_type call_type not null,
    gas_limit BIGINT not null,
    gas_price BIGINT not null,
    gas_used BIGINT not null,
    
    sender character varying(255) not null,
    receiver character varying(255) not null,

    amount BIGINT null,
    asset_id character varying(255) null,

    payload text null,
    payload_data text null,

    timestamp BIGINT null,
    constraint calls_pkey primary key (transaction_id)
  ) tablespace pg_default;