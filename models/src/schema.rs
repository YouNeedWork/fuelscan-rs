// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tx_status"))]
    pub struct TxStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tx_type"))]
    pub struct TxType;
}

diesel::table! {
    accounts (id) {
        id -> Varchar,
        timestamp -> Nullable<Int8>,
    }
}

diesel::table! {
    assets (id) {
        id -> Varchar,
        timestamp -> Nullable<Int8>,
    }
}

diesel::table! {
    blocks (id) {
        id -> Varchar,
        height -> Int8,
        da_height -> Int8,
        application_hash -> Varchar,
        output_messages_root_hash -> Varchar,
        transactions_root -> Varchar,
        prev_root -> Varchar,
        coinbase -> Nullable<Varchar>,
        coinbase_hash -> Nullable<Varchar>,
        coinbase_amount -> Nullable<Varchar>,
        timestamp -> Int8,
        transaction_count -> Int8,
        output_message_count -> Int8,
    }
}

diesel::table! {
    coinbases (id) {
        id -> Varchar,
        height -> Nullable<Int8>,
        da_height -> Nullable<Int8>,
        block_hash -> Varchar,
        amount -> Nullable<Varchar>,
        coinbase -> Nullable<Varchar>,
        timestamp -> Nullable<Int8>,
    }
}

diesel::table! {
    contracts (id) {
        id -> Varchar,
        timestamp -> Nullable<Int4>,
    }
}

diesel::table! {
    nfts (id) {
        id -> Varchar,
        timestamp -> Nullable<Int8>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TxType;
    use super::sql_types::TxStatus;

    transctions (id) {
        id -> Varchar,
        height -> Int8,
        block_hash -> Varchar,
        tx_type -> Nullable<TxType>,
        da_height -> Int8,
        gas_limit -> Varchar,
        gas_price -> Varchar,
        timestamp -> Int8,
        sender -> Nullable<Varchar>,
        status -> Nullable<TxStatus>,
        reason -> Nullable<Varchar>,
        input -> Nullable<Json>,
        output -> Nullable<Json>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    assets,
    blocks,
    coinbases,
    contracts,
    nfts,
    transctions,
);
