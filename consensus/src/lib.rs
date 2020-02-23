// Copyright (c) The Starcoin Core Contributors
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use config::NodeConfig;
use crypto::HashValue;
use futures::channel::oneshot;
use std::convert::TryFrom;
use traits::ChainReader;
use types::block::{Block, BlockHeader, BlockNumber, BlockTemplate};

pub mod dummy;

pub trait ConsensusHeader: TryFrom<Vec<u8>> + Into<Vec<u8>> + std::marker::Unpin {}

pub trait Consensus: std::marker::Unpin {
    fn verify_header(reader: &dyn ChainReader, header: &BlockHeader) -> Result<()>;
    /// Construct block with BlockTemplate
    fn create_block(
        reader: &dyn ChainReader,
        block_template: BlockTemplate,
        cancel: oneshot::Receiver<()>,
    ) -> Result<Block>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
