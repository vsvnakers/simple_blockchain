use crate::blockchain::Blockchain;
use serde_json;
use std::fs;

/// 保存区块链数据
pub fn save_blockchain(blockchain: &Blockchain, filename: &str) {
    let json = serde_json::to_string_pretty(blockchain).unwrap();
    fs::write(filename, json).expect("无法写入文件");
}

/// 读取区块链数据
pub fn load_blockchain(filename: &str) -> Blockchain {
    let data = fs::read_to_string(filename).expect("无法读取文件");
    serde_json::from_str(&data).unwrap()
}
