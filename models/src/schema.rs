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
        #[max_length = 255]
        id -> Varchar,
        timestamp -> Nullable<Int8>,
    }
}

diesel::table! {
    assets (id) {
        #[max_length = 255]
        id -> Varchar,
        timestamp -> Nullable<Int8>,
    }
}

diesel::table! {
    blocks (id) {
        #[max_length = 255]
        id -> Varchar,
        height -> Int8,
        da_height -> Int8,
        #[max_length = 255]
        application_hash -> Varchar,
        #[max_length = 255]
        output_messages_root_hash -> Varchar,
        #[max_length = 255]
        transactions_root -> Varchar,
        #[max_length = 255]
        prev_root -> Varchar,
        #[max_length = 255]
        coinbase -> Nullable<Varchar>,
        #[max_length = 255]
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
        #[max_length = 255]
        transaction_id -> Varchar,
        height -> Int8,
        da_height -> Int8,
        #[max_length = 255]
        block_hash -> Varchar,
        call_type -> CallType,
        gas_limit -> Int8,
        gas_price -> Int8,
        gas_used -> Int8,
        #[max_length = 255]
        sender -> Varchar,
        #[max_length = 255]
        receiver -> Varchar,
        amount -> Nullable<Int8>,
        #[max_length = 255]
        asset_id -> Nullable<Varchar>,
        payload -> Nullable<Text>,
        payload_data -> Nullable<Text>,
        timestamp -> Nullable<Int8>,
    }
}

diesel::table! {
    coinbases (id) {
        #[max_length = 255]
        id -> Varchar,
        height -> Nullable<Int8>,
        da_height -> Nullable<Int8>,
        #[max_length = 255]
        block_hash -> Varchar,
        amount -> Nullable<Int8>,
        #[max_length = 255]
        coinbase -> Nullable<Varchar>,
        timestamp -> Nullable<Int8>,
    }
}

diesel::table! {
    contracts (contract_id) {
        #[max_length = 255]
        contract_id -> Varchar,
        #[max_length = 255]
        transaction_id -> Varchar,
        #[max_length = 255]
        sender -> Varchar,
        bytecode -> Text,
        bytecoin_length -> Int8,
        storage_slots -> Nullable<Json>,
        timestamp -> Int8,
    }
}

diesel::table! {
    nfts (id) {
        #[max_length = 255]
        id -> Varchar,
        timestamp -> Nullable<Int8>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TxType;
    use super::sql_types::TxStatus;

    transactions (id) {
        #[max_length = 255]
        id -> Varchar,
        height -> Int8,
        #[max_length = 255]
        block_hash -> Varchar,
        tx_type -> Nullable<TxType>,
        da_height -> Int8,
        gas_limit -> Int8,
        gas_price -> Int8,
        gas_used -> Int8,
        timestamp -> Int8,
        #[max_length = 255]
        sender -> Varchar,
        status -> TxStatus,
        #[max_length = 255]
        reason -> Varchar,
        input -> Nullable<Json>,
        output -> Nullable<Json>,
        receipts -> Nullable<Json>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    assets,
    blocks,
    calls,
    coinbases,
    contracts,
    nfts,
    transactions,
);
