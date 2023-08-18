## blockchain data indexer


fuelscan backend built by fuelsupply


use tool

- pgsql
- rust-tokio




drop all tables 
```sql
drop table if exists blocks;
drop table if exists assets;
drop table if exists accounts;
drop table if exists check_point;
drop table if exists transactions;
drop table if exists coinbase;
drop table if exists contracts;
drop table if exists nfts;
```
