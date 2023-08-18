use anyhow::Result;
use fuel_core_client::client::{schema::block::Header, types::TransactionResponse};

use crate::block_read::BlockBodies;

use super::blocks::init_block_by_with_header;

pub async fn process(header: &Header, bodies: &BlockBodies) -> Result<()> {
    let block = init_block_by_with_header(header);

    dbg!(bodies);

    Ok(())
}
