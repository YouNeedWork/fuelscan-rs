use fuel_core_client::client::types::block::Header;
use models::block::Block;

pub fn init_block_by_with_header(header: &Header) -> Block {
    Block {
        id: header.id.to_string(),
        height: header.height as i64,
        da_height: header.da_height as i64,
        application_hash: header.application_hash.to_string(),
        output_messages_root_hash: header.message_outbox_root.to_string(),
        transactions_root: header.transactions_root.to_string(),
        prev_root: header.prev_root.to_string(),
        coinbase: None,
        coinbase_hash: None,
        coinbase_amount: None,
        transaction_count: header.transactions_count as i64,
        output_message_count: header.message_receipt_count as i64,
        timestamp: header.time.clone().to_unix(),
    }
}
