-- Your SQL goes here
create table
  blocks (
    id character varying(255) not null,
    height BIGINT not null,
    da_height BIGINT not null,
    application_hash character varying(255) not null,
    output_messages_root_hash character varying(255) not null,
    transactions_root character varying(255) not null,
    prev_root character varying(255) not null,
    coinbase character varying(255) null,
    coinbase_hash character varying(255) null,
    coinbase_amount character varying(255) null,
    timestamp BIGINT not null,
    transaction_count BIGINT not null,
    output_message_count BIGINT not null,
    constraint blocks_pkey primary key (id)
  ) tablespace pg_default;

create index blocks_height_index on blocks (height);
create index blocks_timestamp_index on blocks (timestamp);