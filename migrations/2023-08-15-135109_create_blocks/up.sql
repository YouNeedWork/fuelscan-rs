-- Your SQL goes here
create table
  blocks (
    id varchar not null,
    height BIGINT not null,
    da_height BIGINT not null,
    application_hash varchar not null,
    output_messages_root_hash varchar not null,
    transactions_root varchar not null,
    prev_root varchar not null,
    coinbase varchar null,
    coinbase_hash varchar null,
    coinbase_amount BIGINT  null,
    timestamp BIGINT not null,
    transaction_count BIGINT not null,
    output_message_count BIGINT not null,
    constraint blocks_pkey primary key (id)
  ) tablespace pg_default;

create index blocks_height_index on blocks (height);
create index blocks_timestamp_index on blocks (timestamp);