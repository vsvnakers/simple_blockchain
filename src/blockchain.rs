use crate::block::Block;
use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    /// 初始化区块链（创世区块）
    pub fn new(difficulty: usize) -> Blockchain {
        let genesis_block = Block::new(0, "0".to_string(), vec![], difficulty);
        Blockchain {
            chain: vec![genesis_block],
            difficulty,
        }
    }

    /// 添加新区块
    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            previous_block.hash.clone(),
            transactions,
            self.difficulty,
        );
        self.chain.push(new_block);
    }
}
