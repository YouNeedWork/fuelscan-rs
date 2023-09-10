-- Your SQL goes here
DO
$$
BEGIN
CREATE TYPE asset_status AS ENUM ('alive', 'delete');
END
$$;

create table
  assets (
    assets_utxo_id varchar not null,
    assets_id varchar not null,
    assets_owner varchar not null,
    amount bigint not null, -- if amount only one, maybe is a nft.?
    block_height bigint not null,    
    create_height bigint not null,

    create_tx_hash varchar not null,
    delete_tx_hash varchar not null,
    last_seen timestamp not null,    
    first_seen timestamp not null,
    asset_status asset_status not null,
    
    constraint assets_pkey primary key (assets_utxo_id)
  ) tablespace pg_default;


create index assets_asset_id_index on assets (assets_id);
create index assets_asset_owner_index on assets (assets_owner);
create index assets_delete_tx_hash_index on assets (delete_tx_hash);
create index assets_create_tx_hash_index on assets (create_tx_hash);
create index assets_asset_status_index on assets (asset_status);