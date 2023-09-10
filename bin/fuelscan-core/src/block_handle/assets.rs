use fuel_core_client::client::types::block::Header;
use fuel_core_types::{
    fuel_tx::{
        field::{Inputs, Outputs},
        Create, Mint, Output, Script, Transaction, UniqueIdentifier, UtxoId,
    },
    fuel_types::ChainId,
};

use models::assets::{AssetStatus, Assets};
use rayon::prelude::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator,
    IntoParallelRefMutIterator, ParallelIterator,
};

use crate::block_read::BlockBodies;

use super::CHAIN_ID;

// delete inputs utxo_id
// and store output utxo_id
fn handle_script(s: &Script) -> Option<(Vec<Assets>, Vec<Assets>)> {
    //let mut result = vec![];

    let inputs = s.inputs();
    let outputs = s.outputs();
    let delete_assets = inputs
        .par_iter()
        .filter_map(|i| {
            if i.is_coin_signed() || i.is_coin_predicate() || i.is_coin() {
                let input_coin = i.utxo_id().expect("unreachable");
                let mut asset = Assets::default();

                asset.assets_utxo_id = format!("{:x}", input_coin);
                asset.assets_owner = i
                    .input_owner()
                    .expect("failed find sender with input")
                    .to_string();
                asset.assets_id = i
                    .asset_id()
                    .expect("failed find asset_id with input")
                    .to_string();
                asset.asset_status = AssetStatus::Delete;
                asset.delete_tx_hash = format!("{:x}", s.id(&ChainId::new(CHAIN_ID)));

                Some(asset)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let insert_assets = outputs
        .par_iter()
        .enumerate()
        .filter_map(|(output_index, o)| {
            if o.is_coin()
                || matches!(o, Output::Change { .. })
                || matches!(o, Output::Variable { .. })
            {
                let mut asset = Assets::default();
                asset.assets_id = o
                    .asset_id()
                    .expect("failed find asset_id with output")
                    .to_string();

                asset.assets_utxo_id = format!(
                    "{:x}",
                    UtxoId::new(s.id(&ChainId::new(CHAIN_ID)), output_index as u8)
                );

                asset.assets_owner = o.to().expect("failed find to with output").to_string();
                asset.amount = o.amount().expect("failed find amount with output") as i64;
                Some(asset)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Some((delete_assets, insert_assets))
}

fn handle_create(c: &Create) -> Option<(Vec<Assets>, Vec<Assets>)> {
    let inputs = c.inputs();
    let outputs = c.outputs();
    let delete_assets = inputs
        .par_iter()
        .filter_map(|i| {
            if i.is_coin_signed() || i.is_coin_predicate() || i.is_coin() {
                let input_coin = i.utxo_id().expect("unreachable");
                let mut asset = Assets::default();

                asset.assets_utxo_id = format!("{:x}", input_coin);
                asset.assets_owner = i
                    .input_owner()
                    .expect("failed find sender with input")
                    .to_string();
                asset.assets_id = i
                    .asset_id()
                    .expect("failed find asset_id with input")
                    .to_string();

                asset.asset_status = AssetStatus::Delete;
                asset.delete_tx_hash = format!("{:x}", c.id(&ChainId::new(CHAIN_ID)));

                Some(asset)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let insert_assets = outputs
        .par_iter()
        .enumerate()
        .filter_map(|(output_index, o)| {
            if o.is_coin()
                || matches!(o, Output::Change { .. })
                || matches!(o, Output::Variable { .. })
            {
                let mut asset = Assets::default();
                asset.assets_id = o
                    .asset_id()
                    .expect("failed find asset_id with output")
                    .to_string();

                asset.assets_utxo_id = format!(
                    "{:x}",
                    UtxoId::new(c.id(&ChainId::new(CHAIN_ID)), output_index as u8)
                );
                asset.create_tx_hash = c.id(&ChainId::new(CHAIN_ID)).to_string();
                asset.assets_owner = o.to().expect("failed find to with output").to_string();
                asset.amount = o.amount().expect("failed find amount with output") as i64;
                Some(asset)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Some((delete_assets, insert_assets))
}

fn handle_mint(m: &Mint) -> Option<(Vec<Assets>, Vec<Assets>)> {
    let outputs = m.outputs();

    let insert_assets = outputs
        .par_iter()
        .enumerate()
        .filter_map(|(output_index, o)| {
            if o.is_coin() {
                let mut asset = Assets::default();
                asset.assets_id = o
                    .asset_id()
                    .expect("failed find asset_id with output")
                    .to_string();

                asset.assets_utxo_id = format!(
                    "{:x}",
                    UtxoId::new(m.id(&ChainId::new(CHAIN_ID)), output_index as u8)
                );
                asset.create_tx_hash = m.id(&ChainId::new(CHAIN_ID)).to_string();
                asset.assets_owner = o.to().expect("failed find to with output").to_string();
                asset.amount = o.amount().expect("failed find amount with output") as i64;
                Some(asset)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Some((vec![], insert_assets))
}

pub fn assets_process(header: &Header, bodies: &BlockBodies) -> (Vec<Assets>, Vec<Assets>) {
    let delete_and_insert = bodies
        .par_iter()
        .filter_map(|(_, maybe_tx, _)| {
            maybe_tx.as_ref().and_then(|tx| match &tx.transaction {
                Transaction::Script(s) => handle_script(s),
                Transaction::Create(c) => handle_create(c),
                Transaction::Mint(m) => handle_mint(m),
            })
        })
        .collect::<Vec<(Vec<Assets>, Vec<Assets>)>>();

    let (mut delete, mut insert): (Vec<Assets>, Vec<Assets>) = (vec![], vec![]);
    for (a, b) in delete_and_insert {
        delete.extend(a);
        insert.extend(b);
    }

    delete.par_iter_mut().for_each(|a| {
        a.block_height = header.height as i64;
        a.create_height = header.height as i64;
        //TODO: handle first_seen and last_seen
    });

    insert.par_iter_mut().for_each(|a| {
        a.block_height = header.height as i64;
        a.create_height = header.height as i64;
        //TODO: handle first_seen and last_seen
    });

    (delete, insert)
}
