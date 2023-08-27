-- Your SQL goes here
create table
  assets (
    id bigint,
    asset_id varchar not null,
    asset_hash varchar not null,
    amount bigint not null, -- if amount only one, maybe is a nft.?
    block_height bigint not null,    
    create_height bigint not null,

    create_tx_hash varchar not null,
    last_seen timestamp not null,

    first_seen timestamp not null,
    constraint assets_pkey primary key (id)
  ) tablespace pg_default;