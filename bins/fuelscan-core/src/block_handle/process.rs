use anyhow::Result;
use fuel_core_client::client::{
    schema::block::Header,
    types::{TransactionResponse, TransactionStatus},
};
use fuel_core_types::fuel_tx::{
    field::{
        BytecodeLength, BytecodeWitnessIndex, GasLimit, GasPrice, Inputs, Outputs, StorageSlots,
        Witnesses,
    },
    Receipt,
};
use models::{
    block::Block,
    coinbase::Coinbase,
    contract::Contract,
    transaction::{Transaction, TxStatus, TxType},
};

use crate::block_read::{BlockBodies, BlockBody};

use super::blocks::init_block_by_with_header;

pub async fn process(
    header: &Header,
    bodies: &BlockBodies,
) -> Result<(Block, Option<Coinbase>, Vec<Transaction>, Vec<Contract>)> {
    let mut block = init_block_by_with_header(header);
    let mut coinbase: Option<Coinbase> = None;

    if let Some((tx, coinbase_tx, _)) = coinbase_pick(bodies) {
        block.coinbase_hash = Some(tx.clone());
        if let Some(c) = coinbase_tx.clone().unwrap().transaction.as_mint() {
            block.coinbase_amount = Some(c.outputs()[0].amount().unwrap() as i64);
            block.coinbase = Some(c.outputs()[0].to().unwrap().to_string());
            coinbase = Some(Coinbase {
                id: tx.clone(),
                height: header.height.0 as i64,
                da_height: header.da_height.0 as i64,
                block_hash: header.id.to_string(),
                amount: block.coinbase_amount.clone(),
                coinbase: block.coinbase.clone(),
                timestamp: Some(header.time.0.to_unix()),
            });
        }
    }
    let mut transactions = vec![];
    let mut contracts = vec![];

    let deploy_contract = deploy_contract_transactions(header, bodies);
    for (tx, contract) in deploy_contract {
        transactions.push(tx);
        contracts.push(contract);
    }

    let calls = calls_transactions(header, bodies);
    transactions.extend(calls);

    Ok((block, coinbase, transactions, contracts))
}

pub fn deploy(bodies: &BlockBodies) -> Option<Vec<&BlockBodies>> {
    None
}

pub fn deploy_contract_transactions(
    header: &Header,
    bodies: &BlockBodies,
) -> (Vec<(Transaction, Contract)>) {
    let contract_txs = bodies
        .iter()
        .filter(|tx| tx.1.as_ref().is_some())
        .filter(|tx| tx.1.as_ref().unwrap().transaction.is_create())
        .collect::<Vec<_>>();

    contract_txs
        .iter()
        .map(|(tx_hash, tx, receipts)| {
            dbg!(tx_hash);
            dbg!(tx);
            dbg!(receipts);
            //this is safe we already check
            let create = tx.as_ref().unwrap().transaction.as_create().unwrap();
            let sender = create
                .inputs()
                .iter()
                .find(|t| t.is_coin_signed())
                .and_then(|t| match t {
                    fuel_core_types::fuel_tx::Input::CoinSigned {
                        utxo_id,
                        owner,
                        amount,
                        asset_id,
                        tx_pointer,
                        witness_index,
                        maturity,
                    } => Some(owner.to_string()),
                    _ => None,
                })
                .expect("can't find coin sign");
            let (status, reason) = match tx.clone().unwrap().status {
                TransactionStatus::Submitted { submitted_at } => unreachable!(),
                TransactionStatus::Success {
                    block_id,
                    time,
                    program_state,
                } => (TxStatus::Success, "".to_string()),
                TransactionStatus::SqueezedOut { reason } => unimplemented!(),
                TransactionStatus::Failure {
                    block_id,
                    time,
                    reason,
                    program_state,
                } => (TxStatus::Failed, reason),
            };

            let input = serde_json::to_value(create.inputs()).ok();
            let output = serde_json::to_value(create.outputs()).ok();

            let gas_used = receipts
                .iter()
                .filter(|receipt| match receipt {
                    Receipt::ScriptResult { .. } => true,
                    _ => false,
                })
                .map(|receipt| receipt.gas_used().unwrap())
                .sum::<u64>() as i64;

            (
                Transaction {
                    id: tx_hash.to_string(),
                    height: header.height.0 as i64,
                    da_height: header.da_height.0 as i64,
                    block_hash: header.id.to_string(),
                    tx_type: Some(TxType::Deploy),
                    gas_limit: *create.gas_limit() as i64,
                    gas_price: *create.gas_price() as i64,
                    gas_used,
                    timestamp: header.time.clone().to_unix() as i64,
                    sender: Some(sender.to_string()),
                    status: status,
                    reason: reason,
                    input,
                    output,
                    receipts: serde_json::to_value(receipts).ok(),
                },
                Contract {
                    contract_id: tx_hash.to_string(),
                    transaction_id: tx_hash.to_string(),
                    sender: sender.to_string(),
                    bytecode: hex::encode(
                        create
                            .witnesses()
                            .get(*create.bytecode_witness_index() as usize)
                            .expect("get bytecode: unreachable"),
                    ),
                    bytecoin_length: *create.bytecode_length() as i64,
                    storage_slots: serde_json::to_value(create.storage_slots()).ok(),
                    timestamp: header.time.clone().to_unix() as i64,
                },
            )
        })
        .collect::<Vec<_>>()
}

pub fn calls_transactions(header: &Header, bodies: &BlockBodies) -> Vec<Transaction> {
    let contract_txs = bodies
        .iter()
        .filter(|tx| tx.1.as_ref().is_some())
        .filter(|tx| tx.1.as_ref().unwrap().transaction.is_script())
        .collect::<Vec<_>>();

    contract_txs
        .iter()
        .map(|(tx_hash, tx, receipts)| {
            dbg!(tx_hash);
            dbg!(tx);
            dbg!(receipts);
            //this is safe we already check
            let create = tx
                .as_ref()
                .clone()
                .unwrap()
                .transaction
                .as_script()
                .unwrap();
            let sender = create
                .inputs()
                .iter()
                .find(|t| t.is_coin_signed())
                .and_then(|t| match t {
                    fuel_core_types::fuel_tx::Input::CoinSigned {
                        utxo_id,
                        owner,
                        amount,
                        asset_id,
                        tx_pointer,
                        witness_index,
                        maturity,
                    } => Some(owner.to_string()),
                    _ => None,
                })
                .expect("can't find coin sign");

            let (status, reason) = match tx.clone().unwrap().status {
                TransactionStatus::Submitted { submitted_at } => unreachable!(),
                TransactionStatus::Success {
                    block_id,
                    time,
                    program_state,
                } => (TxStatus::Success, "".to_string()),
                TransactionStatus::SqueezedOut { reason } => unimplemented!(),
                TransactionStatus::Failure {
                    block_id,
                    time,
                    reason,
                    program_state,
                } => (TxStatus::Failed, reason),
            };

            let gas_used = receipts
                .iter()
                .filter(|receipt| match receipt {
                    Receipt::ScriptResult { .. } => true,
                    _ => false,
                })
                .map(|receipt| receipt.gas_used().unwrap())
                .sum::<u64>() as i64;

            let input = serde_json::to_value(create.inputs()).ok();
            let output = serde_json::to_value(create.outputs()).ok();

            Transaction {
                id: tx_hash.to_string(),
                height: header.height.0 as i64,
                da_height: header.da_height.0 as i64,
                block_hash: header.id.to_string(),
                tx_type: Some(TxType::Call),
                gas_limit: *create.gas_limit() as i64,
                gas_price: *create.gas_price() as i64,
                gas_used,
                timestamp: header.time.clone().to_unix() as i64,
                sender: Some(sender.to_string()),
                status: status,
                reason: reason,
                input,
                output,
                receipts: serde_json::to_value(receipts).ok(),
            }
        })
        .collect::<Vec<_>>()
}

pub fn coinbase_pick(bodies: &BlockBodies) -> Option<&BlockBody> {
    bodies
        .iter()
        .find(|tx| tx.1.is_some())
        .iter()
        .find(|tx| tx.1.clone().unwrap().transaction.is_mint())
        .cloned()
}
