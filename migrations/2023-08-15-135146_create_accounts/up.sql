-- Your SQL goes here
create table
  accounts (
    id bigint,
    account_hash varchar not null,
    account_code text null,
    account_name text null,
    account_type text null,-- contract or account?

    last_seen timestamp not null,
    first_seen timestamp not null,
    constraint accounts_pkey primary key (id)
  ) tablespace pg_default;