// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "status"))]
    pub struct Status;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "transaction_type"))]
    pub struct TransactionType;
}

diesel::table! {
    accounts (id) {
        id -> Varchar,
        timestamp -> Nullable<Int4>,
    }
}

diesel::table! {
    assets (id) {
        id -> Varchar,
        timestamp -> Nullable<Int4>,
    }
}

diesel::table! {
    blocks (id) {
        id -> Varchar,
        height -> Int4,
        da_height -> Int4,
        application_hash -> Varchar,
        output_messages_root_hash -> Varchar,
        transactions_root -> Varchar,
        prev_root -> Varchar,
        coinbase -> Nullable<Varchar>,
        coinbase_hash -> Nullable<Varchar>,
        coinbase_amount -> Nullable<Varchar>,
        timestamp -> Int4,
        count -> Int4,
    }
}

diesel::table! {
    check_point (id) {
        id -> Int4,
        height -> Int4,
    }
}

diesel::table! {
    coinbases (id) {
        id -> Varchar,
        height -> Nullable<Int4>,
        da_height -> Nullable<Int4>,
        block_hash -> Varchar,
        amount -> Nullable<Varchar>,
        coinbase -> Nullable<Varchar>,
        timestamp -> Nullable<Int4>,
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
        timestamp -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TransactionType;
    use super::sql_types::Status;

    transctions (id) {
        id -> Varchar,
        height -> Int4,
        block_hash -> Varchar,
        tx_type -> Nullable<TransactionType>,
        da_height -> Int4,
        gas_limit -> Varchar,
        gas_price -> Varchar,
        timestamp -> Int4,
        sender -> Nullable<Varchar>,
        status -> Nullable<Status>,
        reason -> Nullable<Varchar>,
        input -> Nullable<Json>,
        output -> Nullable<Json>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    assets,
    blocks,
    check_point,
    coinbases,
    contracts,
    nfts,
    transctions,
);
