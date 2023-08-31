-- Your SQL goes here
CREATE TABLE address_coin_balances (
    "address_hash" varchar NOT NULL,
    "asset_hash" varchar NOT NULL,
    "block_number" int8 NOT NULL,
    "value" numeric DEFAULT NULL::numeric,
    "value_fetched_at" timestamp,
    "inserted_at" timestamp NOT NULL,
    "updated_at" timestamp NOT NULL,
    PRIMARY KEY ("address_hash","asset_hash","block_number")
);