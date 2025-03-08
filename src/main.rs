use std::fs;
mod block;
mod blockchain;
mod cli;
mod storage;
mod transaction;

use blockchain::Blockchain;

fn main() {
    let filename = "blockchain.json";

    // 读取或创建区块链
    let mut blockchain = if fs::metadata(filename).is_ok() {
        println!("🔄 加载现有区块链数据...");
        storage::load_blockchain(filename)
    } else {
        println!("🆕 创建新的区块链...");
        Blockchain::new(4)
    };

    // 显示创世区块
    let genesis_block = &blockchain.chain[0];
    println!("🚀 创世区块: {:#?}", genesis_block);

    // ✅ 调用 `run_cli()`，进入命令行交互模式
    cli::run_cli(&mut blockchain, filename);
}
