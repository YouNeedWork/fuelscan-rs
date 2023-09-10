## blockchain data indexer


fuelscan backend built by fuelsupply


use tool

- pgsql
- rust-tokio




drop all tables 
```sql
drop table if exists calls;
drop table if exists blocks;
drop table if exists assets;
drop table if exists accounts;
drop table if exists check_point;
drop table if exists transactions;
drop table if exists coinbases;
drop table if exists contracts;
drop table if exists nfts;
drop table if exists __diesel_schema_migrations;
drop table if exists address_coin_balances;
drop table if exists address_coin_balances_daily;
drop table if exists smart_contracts;

drop type if exists tx_type;
drop type if exists tx_status;
drop type if exists call_type;
drop type if exists asset_status;
```
