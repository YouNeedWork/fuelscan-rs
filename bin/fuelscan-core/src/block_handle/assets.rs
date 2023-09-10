use anyhow::Result;
use fuel_core_client::client::types::block::Header;
use fuel_core_types::fuel_tx::{
    field::{Inputs, Outputs},
    Create, Script, Transaction,
};
use models::assets::Assets;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::block_read::BlockBodies;

fn handle_script(s: &Script) -> Option<Assets> {
    let mut result = vec![];

    let inputs = s.inputs();
    let outputs = s.outputs();
    let gas_asset = inputs
        .par_iter()
        .filter_map(|i| {
            if i.is_coin_signed() || i.is_coin_predicate() || i.is_coin() {
                let input_coin = i.utxo_id().expect("unreachable");
                let mut asset = Assets::default();
                let output_index = input_coin.output_index();
                asset.assets_id = input_coin.tx_id().to_string();
                asset.assets_hash = i
                    .asset_id()
                    .expect("failed find asset_id with input")
                    .to_string();
                let output = outputs.get(output_index as usize).expect("unreachable");
                asset.amount = output.amount().expect("failed find amount with output") as i64;
                Some(asset)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    result.extend(gas_asset);
    let create_coin = outputs
        .par_iter()
        .filter_map(|o| {
            if o.is_coin() {
                let mut asset = Assets::default();
                asset.assets_hash = o
                    .asset_id()
                    .expect("failed find asset_id with output")
                    .to_string();
                asset.amount = o.amount().expect("failed find amount with output") as i64;
                Some(asset)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    result.extend(create_coin);

    dbg!(result);
    None
}

fn handle_create(c: &Create) -> Option<Assets> {
    let inputs = c.inputs();
    let outputs = c.outputs();
    inputs.par_iter().for_each(|i| {
        println!("input: {:?}", i);
        outputs.par_iter().for_each(|o| {
            println!("output: {:?}", o);
        });
    });
    None
}

pub fn assets_process(header: &Header, bodies: &BlockBodies) -> Result<Vec<Assets>> {
    let result = bodies
        .par_iter()
        .filter_map(|(_, maybe_tx, _)| {
            maybe_tx.as_ref().and_then(|tx| match &tx.transaction {
                Transaction::Script(s) => handle_script(s),
                Transaction::Create(c) => handle_create(c),
                Transaction::Mint(_) => None,
            })
        })
        .collect::<Vec<Assets>>();

    Ok(result)
}
