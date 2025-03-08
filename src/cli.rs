use crate::blockchain::Blockchain;
use crate::storage;
use crate::transaction::Transaction;
use std::io::{self, Write};

/// **命令行交互函数**
pub fn run_cli(blockchain: &mut Blockchain, filename: &str) {
    loop {
        print!("\n请输入命令 (check / mining / view / exit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "check" => {
                println!("🔍 检查区块链完整性...");
                if check_blockchain(blockchain) {
                    println!("✅ 区块链完整且有效！");
                } else {
                    println!("❌ 区块链数据异常！");
                }
            }
            "mining" => {
                println!("⛏️ 正在挖矿...");
                let tx1 = Transaction::new("Alice".to_string(), "Bob".to_string(), 10);
                let tx2 = Transaction::new("Bob".to_string(), "Charlie".to_string(), 5);
                blockchain.add_block(vec![tx1, tx2]);
                println!("✅ 新区块已添加！");
                storage::save_blockchain(blockchain, filename);
            }
            "view" => {
                println!("📜 当前区块链数据:");
                for block in &blockchain.chain {
                    println!("{:#?}", block);
                }
            }
            "exit" => {
                println!("👋 退出程序...");
                break;
            }
            _ => {
                println!("⚠️ 无效命令，请输入 check / enter / view / exit");
            }
        }
    }
}

/// **区块链完整性检查**
fn check_blockchain(blockchain: &Blockchain) -> bool {
    for i in 1..blockchain.chain.len() {
        let prev_block = &blockchain.chain[i - 1];
        let current_block = &blockchain.chain[i];

        // 1. 检查前一个区块哈希是否匹配
        if current_block.previous_hash != prev_block.hash {
            println!("❌ 区块 {} 的前一个哈希不匹配！", current_block.index);
            return false;
        }

        // 2. 重新计算哈希，看是否匹配
        let recalculated_hash = crate::block::Block::calculate_hash(
            current_block.index,
            current_block.timestamp,
            &current_block.previous_hash,
            current_block.nonce,
            &current_block.transactions,
        );
        if current_block.hash != recalculated_hash {
            println!("❌ 区块 {} 的哈希不正确！", current_block.index);
            return false;
        }
    }
    true
}
