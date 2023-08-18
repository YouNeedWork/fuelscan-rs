-- Your SQL goes here
DO
$$
BEGIN
CREATE TYPE tx_type AS ENUM ('call', 'deploy');
CREATE TYPE tx_status AS ENUM ('success', 'failed');
END
$$;

create table
  transctions (
    id character varying(255) not null,
    height BIGINT not null,
    block_hash character varying(255) not null,
    tx_type tx_type null,
    da_height BIGINT not null,
    gas_limit character varying(255) not null,
    gas_price character varying(255) not null,
    timestamp BIGINT not null,
    sender character varying(255) null,
    status tx_status null,
    reason character varying(255) null,
    input json null,
    output json null,
    constraint transctions_pkey primary key (id)
  ) tablespace pg_default;

create index transctions_height_index on transctions (height);
create index transctions_block_hash_index on transctions (block_hash);
create index timestamp_index on transctions (timestamp);
