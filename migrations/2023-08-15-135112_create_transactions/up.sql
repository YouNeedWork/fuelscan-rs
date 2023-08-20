-- Your SQL goes here
DO
$$
BEGIN
CREATE TYPE tx_type AS ENUM ('call', 'deploy');
CREATE TYPE tx_status AS ENUM ('success', 'failed');
END
$$;

create table
  transactions (
    id character varying(255) not null,
    height BIGINT not null,
    block_hash character varying(255) not null,
    tx_type tx_type null,
    da_height BIGINT not null,
    gas_limit BIGINT not null,
    gas_price BIGINT not null,
    gas_used BIGINT not null,
    timestamp BIGINT not null,
    sender character varying(255) not null,
    status tx_status not null,
    reason character varying(255) not null,
    input json null,
    output json null,
    receipts json null,
    constraint transactions_pkey primary key (id)
  ) tablespace pg_default;

create index transactions_height_index on transactions (height);
create index transactions_block_hash_index on transactions (block_hash);
create index timestamp_index on transactions (timestamp);
