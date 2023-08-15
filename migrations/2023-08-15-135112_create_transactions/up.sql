-- Your SQL goes here
create table
  transctions (
    id character varying(255) not null,
    height integer not null,
    block_hash character varying(255) not null,
    tx_type public.transaction_type null,
    da_height integer not null,
    gas_limit character varying(255) not null,
    gas_price character varying(255) not null,
    timestamp integer not null,
    sender character varying(255) null,
    status public.status null,
    reason character varying(255) null,
    input json null,
    output json null,
    constraint transctions_pkey primary key (id)
  ) tablespace pg_default;

create index transctions_height_index on transctions (height);
create index transctions_block_hash_index on transctions (block_hash);
create index timestamp_index on transctions (timestamp);
