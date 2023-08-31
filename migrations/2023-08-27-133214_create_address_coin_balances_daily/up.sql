-- Your SQL goes here
CREATE TABLE address_coin_balances_daily (
    "address_hash" varchar NOT NULL,
    "asset_hash" varchar NOT NULL, 
    "day" date NOT NULL,
    "value" numeric DEFAULT NULL::numeric,
    "inserted_at" timestamp NOT NULL,
    "updated_at" timestamp NOT NULL,
    PRIMARY KEY ("address_hash","asset_hash","day")
);