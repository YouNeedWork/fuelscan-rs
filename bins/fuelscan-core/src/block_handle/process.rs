use anyhow::Result;
use fuel_core_client::client::{schema::block::Header, types::TransactionStatus};
use fuel_core_types::fuel_tx::{
    field::{
        BytecodeLength, BytecodeWitnessIndex, GasLimit, GasPrice, Inputs, Outputs, Script,
        ScriptData, StorageSlots, Witnesses,
    },
    Receipt,
};

use models::{
    block::Block,
    call::{Call, CallType},
    coinbase::Coinbase,
    contract::Contract,
    transaction::{Transaction, TxStatus, TxType},
};

use crate::{
    block_handle::add_0x_prefix,
    block_read::{BlockBodies, BlockBody},
};

use super::blocks::init_block_by_with_header;

pub async fn process(
    header: &Header,
    bodies: &BlockBodies,
) -> Result<(
    Block,
    Option<Coinbase>,
    Vec<Transaction>,
    Vec<Contract>,
    Vec<Call>,
)> {
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
    let mut calls = vec![];

    let deploy_contract = deploy_contract_transactions(header, bodies);
    for (tx, contract) in deploy_contract {
        transactions.push(tx);
        contracts.push(contract);
    }

    let calls_contract = calls_transactions(header, bodies);
    for (tx, call) in calls_contract {
        transactions.push(tx);
        calls.push(call);
    }

    Ok((block, coinbase, transactions, contracts, calls))
}

pub fn deploy_contract_transactions(
    header: &Header,
    bodies: &BlockBodies,
) -> Vec<(Transaction, Contract)> {
    //genesis block don't give any response
    if header.transactions_count.0 == 0 {
        return vec![];
    };

    let contract_txs = bodies
        .iter()
        .filter(|tx| tx.1.as_ref().is_some())
        .filter(|tx| tx.1.as_ref().unwrap().transaction.is_create())
        .collect::<Vec<_>>();

    contract_txs
        .iter()
        .map(|(tx_hash, tx, receipts)| {
            //this is safe we already check
            let create = tx.as_ref().unwrap().transaction.as_create().unwrap();
            let sender = create
                .inputs()
                .iter()
                .find(|t| t.is_coin_signed())
                .and_then(|t| match t {
                    fuel_core_types::fuel_tx::Input::CoinSigned {
                        utxo_id: _,
                        owner,
                        amount: _,
                        asset_id: _,
                        tx_pointer: _,
                        witness_index: _,
                        maturity: _,
                    } => Some(add_0x_prefix(owner.to_string())),
                    _ => None,
                })
                .expect("can't find coin sign");
            let (status, reason) = match tx.clone().unwrap().status {
                TransactionStatus::Submitted { submitted_at: _ } => unreachable!(),
                TransactionStatus::Success {
                    block_id: _,
                    time: _,
                    program_state: _,
                } => (TxStatus::Success, "".to_string()),
                TransactionStatus::SqueezedOut { reason: _ } => unimplemented!(),
                TransactionStatus::Failure {
                    block_id: _,
                    time: _,
                    reason,
                    program_state: _,
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
                    sender: Some(sender.clone()),
                    status: status,
                    reason: reason,
                    input,
                    output,
                    receipts: serde_json::to_value(receipts).ok(),
                },
                Contract {
                    contract_id: tx_hash.to_string(),
                    transaction_id: tx_hash.to_string(),
                    sender,
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

pub fn calls_transactions(header: &Header, bodies: &BlockBodies) -> Vec<(Transaction, Call)> {
    //genesis block don't give any response
    if header.transactions_count.0 == 0 {
        return vec![];
    };

    let contract_txs = bodies
        .iter()
        .filter(|tx| tx.1.as_ref().is_some())
        .filter(|tx| tx.1.as_ref().unwrap().transaction.is_script())
        .collect::<Vec<_>>();

    contract_txs
        .iter()
        .map(|(tx_hash, tx, receipts)| {
            //this is safe we already check
            let call = tx
                .as_ref()
                .clone()
                .unwrap()
                .transaction
                .as_script()
                .unwrap();
            let (sender, signed_asset_id) = call
                .inputs()
                .iter()
                .find(|t| t.is_coin_signed())
                .and_then(|t| match t {
                    fuel_core_types::fuel_tx::Input::CoinSigned {
                        owner, asset_id, ..
                    } => Some((add_0x_prefix(owner.to_string()), asset_id)),
                    _ => None,
                })
                .expect("can't find coin sign");

            let (status, reason) = match tx.clone().unwrap().status {
                TransactionStatus::Submitted { submitted_at: _ } => unreachable!(),
                TransactionStatus::Success {
                    block_id: _,
                    time: _,
                    program_state: _,
                } => (TxStatus::Success, "".to_string()),
                TransactionStatus::SqueezedOut { reason: _ } => unimplemented!(),
                TransactionStatus::Failure {
                    block_id: _,
                    time: _,
                    reason,
                    program_state: _,
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

            let input = serde_json::to_value(call.inputs()).ok();
            let output = serde_json::to_value(call.outputs()).ok();

            let (call_type, amount, asset_id, to, payload, payload_data) =
                if let Some(_) = call.outputs().iter().find(|t| t.is_contract()) {
                    let payload = call.script();
                    let payload_data = call.script_data();

                    match receipts
                        .iter()
                        .find(|receipt| match receipt {
                            Receipt::Call { .. } => true,
                            _ => false,
                        })
                        .expect("can't find coin output")
                    {
                        Receipt::Call {
                            amount,
                            asset_id,
                            to,
                            id: _,
                            gas: _,
                            param1: _,
                            param2: _,
                            pc: _,
                            is: _,
                        } => (
                            CallType::Contract,
                            Some(*amount as i64),
                            Some(asset_id.to_string()),
                            add_0x_prefix(to.to_string()),
                            Some(hex::encode(payload)),
                            Some(hex::encode(payload_data)),
                        ),
                        _ => unreachable!(),
                    }
                /*                     (
                    CallType::Contract,
                    Some(*amount as i64),
                    Some(id.to_string()),
                    to.to_string(),
                    None,
                    None,
                ) */
                } else {
                    let (amount, id, to) = call
                        .outputs()
                        .iter()
                        .find(|t| t.is_coin() || t.is_message())
                        .and_then(|t| match t {
                            //FIX:: this can be contract call or simple transfer
                            fuel_core_types::fuel_tx::Output::Coin {
                                amount,
                                asset_id,
                                to,
                            } => Some((amount, asset_id, to.to_string())),
                            //I think this is only  for the transfer ?? why this call message ?
                            fuel_core_types::fuel_tx::Output::Message { amount, recipient } => {
                                Some((amount, signed_asset_id, recipient.to_string()))
                            }
                            _ => None,
                        })
                        .expect("can't find coin output");

                    (
                        CallType::Transaction,
                        Some(*amount as i64),
                        Some(id.to_string()),
                        add_0x_prefix(to.to_string()),
                        None,
                        None,
                    )
                };

            (
                Transaction {
                    id: tx_hash.to_string(),
                    height: header.height.0 as i64,
                    da_height: header.da_height.0 as i64,
                    block_hash: header.id.to_string(),
                    tx_type: Some(TxType::Call),
                    gas_limit: *call.gas_limit() as i64,
                    gas_price: *call.gas_price() as i64,
                    gas_used,
                    timestamp: header.time.clone().to_unix() as i64,
                    sender: Some(sender.to_string()),
                    status: status,
                    reason: reason,
                    input,
                    output,
                    receipts: serde_json::to_value(receipts).ok(),
                },
                Call {
                    transaction_id: tx_hash.to_string(),
                    height: header.height.0 as i64,
                    da_height: header.da_height.0 as i64,
                    block_hash: header.id.to_string(),
                    call_type,
                    gas_limit: *call.gas_limit() as i64,
                    gas_price: *call.gas_price() as i64,
                    gas_used,
                    sender: sender.to_string(),
                    receiver: to,
                    amount,
                    asset_id,
                    payload,
                    payload_data,
                    timestamp: header.time.clone().to_unix() as i64,
                },
            )
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
