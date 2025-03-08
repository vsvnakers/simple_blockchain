use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

/// 区块结构
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>, // ✅ 交易数据
}

impl Block {
    /// 计算区块哈希
    pub fn calculate_hash(
        index: u64,
        timestamp: u128,
        previous_hash: &str,
        nonce: u64,
        transactions: &Vec<Transaction>,
    ) -> String {
        let input = format!(
            "{}{}{}{}{:?}",
            index, timestamp, previous_hash, nonce, transactions
        );
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }

    /// 创建新区块（挖矿）
    pub fn new(
        index: u64,
        previous_hash: String,
        transactions: Vec<Transaction>,
        difficulty: usize,
    ) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let mut nonce = 0;
        let mut hash =
            Block::calculate_hash(index, timestamp, &previous_hash, nonce, &transactions);

        // 挖矿：找到符合难度的哈希
        while &hash[..difficulty] != "0".repeat(difficulty) {
            nonce += 1;
            hash = Block::calculate_hash(index, timestamp, &previous_hash, nonce, &transactions);
        }

        Block {
            index,
            timestamp,
            previous_hash,
            hash,
            nonce,
            transactions, // ✅ 交易数据
        }
    }
}
