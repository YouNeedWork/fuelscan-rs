// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "call_type"))]
    pub struct CallType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tx_status"))]
    pub struct TxStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tx_type"))]
    pub struct TxType;
}

diesel::table! {
    accounts (id) {
        id -> Int8,
        account_hash -> Varchar,
        account_code -> Nullable<Text>,
        account_name -> Nullable<Text>,
        account_type -> Nullable<Text>,
        verified -> Bool,
        gas_used -> Int8,
        transactions_count -> Int8,
        token_transfers_count -> Int8,
        sender_count -> Int8,
        recever_count -> Int8,
        decompiled -> Bool,
        inserted_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    address_coin_balances (address_hash, asset_hash, block_number) {
        address_hash -> Varchar,
        asset_hash -> Varchar,
        block_number -> Int8,
        value -> Nullable<Numeric>,
        value_fetched_at -> Nullable<Timestamp>,
        inserted_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    address_coin_balances_daily (address_hash, asset_hash, day) {
        address_hash -> Varchar,
        asset_hash -> Varchar,
        day -> Date,
        value -> Nullable<Numeric>,
        inserted_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    assets (id) {
        id -> Int8,
        assets_id -> Varchar,
        assets_hash -> Varchar,
        amount -> Int8,
        block_height -> Int8,
        create_height -> Int8,
        create_tx_hash -> Varchar,
        last_seen -> Timestamp,
        first_seen -> Timestamp,
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
        coinbase_amount -> Nullable<Int8>,
        timestamp -> Int8,
        transaction_count -> Int8,
        output_message_count -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CallType;

    calls (transaction_id) {
        transaction_id -> Varchar,
        height -> Int8,
        da_height -> Int8,
        block_hash -> Varchar,
        call_type -> CallType,
        gas_limit -> Int8,
        gas_price -> Int8,
        gas_used -> Int8,
        sender -> Varchar,
        receiver -> Varchar,
        amount -> Nullable<Int8>,
        asset_id -> Nullable<Varchar>,
        payload -> Nullable<Text>,
        payload_data -> Nullable<Text>,
        timestamp -> Nullable<Int8>,
    }
}

diesel::table! {
    coinbases (id) {
        id -> Varchar,
        height -> Nullable<Int8>,
        da_height -> Nullable<Int8>,
        block_hash -> Varchar,
        amount -> Nullable<Int8>,
        coinbase -> Nullable<Varchar>,
        timestamp -> Nullable<Int8>,
    }
}

diesel::table! {
    nfts (id) {
        id -> Varchar,
        timestamp -> Nullable<Int8>,
    }
}

diesel::table! {
    smart_contracts (contract_hash) {
        contract_hash -> Varchar,
        transaction_id -> Varchar,
        sender -> Varchar,
        bytecode -> Text,
        bytecoin_length -> Int8,
        storage_slots -> Nullable<Json>,
        timestamp -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TxType;
    use super::sql_types::TxStatus;

    transactions (id) {
        id -> Varchar,
        height -> Int8,
        block_hash -> Varchar,
        tx_type -> Nullable<TxType>,
        da_height -> Int8,
        gas_limit -> Int8,
        gas_price -> Int8,
        gas_used -> Int8,
        timestamp -> Int8,
        sender -> Varchar,
        status -> TxStatus,
        reason -> Varchar,
        input -> Nullable<Json>,
        output -> Nullable<Json>,
        receipts -> Nullable<Json>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    address_coin_balances,
    address_coin_balances_daily,
    assets,
    blocks,
    calls,
    coinbases,
    nfts,
    smart_contracts,
    transactions,
);
