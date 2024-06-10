use anyhow::Result;
use fuel_core_client::client::types::{block::Header, TransactionStatus};
use fuel_core_types::fuel_tx::{
    field::{
        BytecodeWitnessIndex, Inputs, MaxFeeLimit, MintAmount, MintGasPrice, Outputs, Script,
        ScriptData, StorageSlots, Witnesses,
    },
    input::coin::Coin,
    Input, Receipt,
};

use models::{
    assets::Assets,
    block::Block,
    call::{Call, CallType},
    coinbase::Coinbase,
    contract::Contract,
    transaction::{Transaction, TxStatus, TxType},
};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::block_read::{BlockBodies, BlockBody};

use super::{assets::assets_process, blocks::init_block_by_with_header};

pub async fn process(
    header: &Header,
    bodies: &BlockBodies,
) -> Result<(
    Block,
    Option<Coinbase>,
    Vec<Transaction>,
    Vec<Contract>,
    Vec<Call>,
    (Vec<Assets>, Vec<Assets>),
)> {
    let mut block = init_block_by_with_header(header);
    let mut coinbase: Option<Coinbase> = None;

    if let Some((tx, coinbase_tx, _)) = coinbase_pick(bodies) {
        block.coinbase_hash = Some(tx.to_string());
        if let Some(c) = coinbase_tx.clone().unwrap().transaction.as_mint() {
            block.coinbase_amount = Some(*c.mint_amount() as i64);
            //block.coinbase = Some(c.outputs()[0].to().unwrap().to_string());
            block.coinbase = None;

            coinbase = Some(Coinbase {
                id: tx.to_string(),
                height: header.height as i64,
                da_height: header.da_height as i64,
                block_hash: header.id.to_string(),
                amount: block.coinbase_amount,
                coinbase: block.coinbase.clone(),
                timestamp: Some(header.time.0 as i64),
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

    let assrts = assets_process(header, bodies);

    Ok((block, coinbase, transactions, contracts, calls, assrts))
}

pub fn deploy_contract_transactions(
    header: &Header,
    bodies: &BlockBodies,
) -> Vec<(Transaction, Contract)> {
    //genesis block don't give any response
    if header.transactions_count == 0 {
        return vec![];
    };

    let contract_txs = bodies
        .par_iter()
        .filter(|tx| tx.1.as_ref().is_some())
        .filter(|tx| tx.1.as_ref().unwrap().transaction.is_create())
        .collect::<Vec<_>>();

    contract_txs
        .par_iter()
        .map(|(tx_hash, tx, receipts)| {
            //this is safe we already check
            let create = tx.as_ref().unwrap().transaction.as_create().unwrap();
            let sender = find_sender(create);
            let (status, reason) = match tx.clone().unwrap().status {
                TransactionStatus::Submitted { submitted_at: _ } => unreachable!(),
                TransactionStatus::Success {
                    time: _,
                    program_state: _,
                    receipts: _,
                    block_height: _,
                    total_gas: _,
                    total_fee: _,
                } => (TxStatus::Success, "".to_string()),
                TransactionStatus::SqueezedOut { reason: _ } => unimplemented!(),
                TransactionStatus::Failure {
                    block_height: _,
                    time: _,
                    reason,
                    program_state: _,
                    receipts: _,
                    total_gas: _,
                    total_fee: _,
                } => (TxStatus::Failed, reason),
            };

            let input = serde_json::to_value(create.inputs()).ok();
            let output = serde_json::to_value(create.outputs()).ok();
            let receipts = receipts.as_ref().expect("TODO: There is no receipt");
            let gas_used = receipts
                .par_iter()
                .filter(|receipt| matches!(receipt, Receipt::ScriptResult { .. }))
                .map(|receipt| receipt.gas_used().unwrap())
                .sum::<u64>() as i64;
            (
                Transaction {
                    id: tx_hash.to_string(),
                    height: header.height as i64,
                    da_height: header.da_height as i64,
                    block_hash: header.id.to_string(),
                    tx_type: Some(TxType::Deploy),
                    gas_limit: create.max_fee_limit() as i64,
                    gas_price: 0,
                    gas_used,
                    timestamp: header.time.clone().to_unix(),
                    sender: Some(sender.clone()),
                    status,
                    reason,
                    input,
                    output,
                    receipts: serde_json::to_value(receipts).ok(),
                },
                Contract {
                    contract_hash: tx_hash.to_string(),
                    transaction_id: tx_hash.to_string(),
                    sender,
                    bytecode: hex::encode(
                        create
                            .witnesses()
                            .get(*create.bytecode_witness_index() as usize)
                            .expect("get bytecode: unreachable"),
                    ),
                    bytecoin_length: *create.bytecode_witness_index() as i64,
                    storage_slots: serde_json::to_value(create.storage_slots()).ok(),
                    timestamp: header.time.clone().to_unix(),
                },
            )
        })
        .collect::<Vec<_>>()
}

fn find_sender(create: &fuel_core_types::fuel_tx::Create) -> String {
    //TODO add more select contation in here
    let sender = create
        .inputs()
        .par_iter()
        .find_first(|t| t.is_coin_signed())
        .and_then(|t| match t {
            Input::CoinSigned(Coin { owner, .. }) => Some(owner.to_string()),
            /*                     Input::CoinPredicate(_) => todo!(),
                    Input::Contract(_) => todo!(),
                    Input::MessageCoinSigned(_) => todo!(),
                    Input::MessageCoinPredicate(_) => todo!(),
                    Input::MessageDataSigned(_) => todo!(),
                    Input::MessageDataPredicate(_) => todo!(), */
            /* Input::Signed({
                        utxo_id: _,
                        owner,
                        amount: _,
                        asset_id: _,
                        tx_pointer: _,
                        witness_index: _,
                        maturity: _,
                    }) => Some(owner.to_string()), */
            _ => None,
        })
        .expect("can't find coin sign");
    sender
}

pub fn calls_transactions(header: &Header, bodies: &BlockBodies) -> Vec<(Transaction, Call)> {
    //genesis block don't give any response
    if header.transactions_count == 0 {
        return vec![];
    };

    let contract_txs = bodies
        .par_iter()
        .filter(|tx| tx.1.as_ref().is_some())
        .filter(|tx| tx.1.as_ref().unwrap().transaction.is_script())
        .collect::<Vec<_>>();

    contract_txs
        .par_iter()
        .map(|(tx_hash, tx, receipts)| {
            //this is safe we already check
            let call = tx.as_ref().unwrap().transaction.as_script().unwrap();

            let (sender, _signed_asset_id) = call
                .inputs()
                .par_iter()
                .find_first(|t| t.is_coin_signed() || t.is_coin_predicate())
                .and_then(|t| match t {
                    Input::CoinSigned(Coin {
                        owner, asset_id, ..
                    }) => Some((owner.to_string(), asset_id)),
                    Input::CoinPredicate(Coin {
                        owner, asset_id, ..
                    }) => Some((owner.to_string(), asset_id)),
                    /*                     Input::MessageCoinSigned(Coin { recipient, .. }) => {
                        Some((recipient.to_string(), &AssetId::BASE))
                    } */
                    _ => None,
                })
                .expect("can't find coin signer"); //TODO maybe there have more when one? We need find all of them and fingout what's gas,

            let (status, reason) = match tx.clone().unwrap().status {
                TransactionStatus::Submitted { submitted_at: _ } => unreachable!(),
                TransactionStatus::Success {
                    time: _,
                    program_state: _,
                    receipts: _,
                    block_height,
                    total_gas,
                    total_fee,
                } => (TxStatus::Success, "".to_string()),
                TransactionStatus::SqueezedOut { reason: _ } => unimplemented!(),
                TransactionStatus::Failure {
                    time: _,
                    reason,
                    program_state: _,
                    receipts: _,
                    block_height,
                    total_gas,
                    total_fee,
                } => (TxStatus::Failed, reason),
            };

            let receipts = receipts.as_ref().expect("TODO: There is no receipt");
            let gas_used = receipts
                .par_iter()
                .filter(|receipt| matches!(receipt, Receipt::ScriptResult { .. }))
                .map(|receipt| receipt.gas_used().unwrap())
                .sum::<u64>() as i64;

            let input = serde_json::to_value(call.inputs()).ok();
            let output = serde_json::to_value(call.outputs()).ok();

            let (call_type, amount, asset_id, to, payload, payload_data) = if call
                .outputs()
                .par_iter()
                .any(|t| t.is_contract() || t.is_contract_created())
            {
                let payload = call.script();
                let payload_data = call.script_data();
                match receipts
                    .par_iter()
                    .find_first(|receipt| match receipt {
                        Receipt::Call { .. } => true,
                        Receipt::Transfer { .. } => true,

                        //there is also contract created but maybe failed.
                        //we shuld check this and put in the tarnscation table or what??
                        _ => true,
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
                        to.to_string(),
                        Some(hex::encode(payload)),
                        Some(hex::encode(payload_data)),
                    ),
                    Receipt::Transfer {
                        id,
                        //to,
                        amount,
                        asset_id,
                        ..
                    } => (
                        CallType::Contract,
                        Some(*amount as i64),
                        Some(asset_id.to_string()),
                        id.to_string(),
                        Some(hex::encode(payload)),
                        Some(hex::encode(payload_data)),
                    ),
                    _ => (CallType::Contract, None, None, "".to_string(), None, None),
                }
            } else {
                let (amount, id, to) = match call
                    .outputs()
                    .par_iter()
                    .find_first(|t| t.is_coin() || t.is_variable() || t.is_contract_created())
                    .and_then(|t| match t {
                        //FIX:: this can be contract call or simple transfer
                        //transfer to gas coin?
                        fuel_core_types::fuel_tx::Output::Coin {
                            amount,
                            asset_id,
                            to,
                        } => Some((amount, asset_id, to.to_string())),
                        //transfer other asserts
                        fuel_core_types::fuel_tx::Output::Variable {
                            amount,
                            asset_id,
                            to,
                        } => Some((amount, asset_id, to.to_string())),
                        //I think this is only  for the transfer ?? why this call message ?
                        /* fuel_core_types::fuel_tx::Output::Message { amount, recipient } => {
                            Some((amount, signed_asset_id, recipient.to_string()))
                        } */
                        _ => None,
                    }) {
                    Some((amount, asset_id, address)) => {
                        (Some(*amount as i64), Some(asset_id.to_string()), address)
                    }
                    None => (None, None, "".to_string()),
                };

                let payload = if !call.script().is_empty() {
                    Some(hex::encode(call.script()))
                } else {
                    None
                };

                let payload_data = if !call.script_data().is_empty() {
                    Some(hex::encode(call.script_data()))
                } else {
                    None
                };

                (
                    CallType::Transaction,
                    amount,
                    id,
                    to.to_string(),
                    payload,
                    payload_data,
                )
            };

            (
                Transaction {
                    id: tx_hash.to_string(),
                    height: header.height as i64,
                    da_height: header.da_height as i64,
                    block_hash: header.id.to_string(),
                    tx_type: Some(TxType::Call),
                    gas_limit: call.max_fee_limit() as i64,
                    gas_price: 0,
                    gas_used,
                    timestamp: header.time.clone().to_unix(),
                    sender: Some(sender.to_string()),
                    status,
                    reason,
                    input,
                    output,
                    receipts: serde_json::to_value(receipts).ok(),
                },
                Call {
                    transaction_id: tx_hash.to_string(),
                    height: header.height as i64,
                    da_height: header.da_height as i64,
                    block_hash: header.id.to_string(),
                    call_type,
                    gas_limit: call.max_fee_limit() as i64,
                    gas_price: 0,
                    gas_used,
                    sender: sender.to_string(),
                    receiver: to,
                    amount,
                    asset_id,
                    payload,
                    payload_data,
                    timestamp: header.time.clone().to_unix(),
                },
            )
        })
        .collect::<Vec<_>>()
}

pub fn coinbase_pick(bodies: &BlockBodies) -> Option<&BlockBody> {
    bodies
        .par_iter()
        .find_first(|tx| tx.1.is_some())
        .par_iter()
        .find_first(|tx| tx.1.clone().unwrap().transaction.is_mint())
        .cloned()
}
