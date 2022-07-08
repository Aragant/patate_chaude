use md5;
use rand::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashInput {
    // complexity in bits
    pub complexity: u32,
    // message to sign
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashOutput {
    // Seed used to solve the challenge
    pub seed: u64,
    // hashcode found using seed + message
    pub hashcode: String,
}

pub fn hashcash(input: &MD5HashCashInput) -> MD5HashCashOutput {
    loop
    {
        let seed: u64 = random();
        let mut seed_hexa = format!("{:x}", seed).to_string().to_uppercase();
        seed_hexa = format!("{:0>16}", seed_hexa);
        let seed_with_message=  seed_hexa + &input.message;
        let hashcode_128 = compute_md5_to_u128(seed_with_message);
        let current_complexity = compute_complexity(hashcode_128);
        let mut hashcode = format!("{:x}", hashcode_128).to_string().to_uppercase();
        hashcode = format!("{:0>32}", hashcode);
        if current_complexity >= input.complexity{
            return MD5HashCashOutput {
                seed,
                hashcode
            }
        }
    }
}

fn compute_complexity(hashcode: u128) -> u32 {
    hashcode.leading_zeros()
}

fn compute_md5_to_u128(message: String) -> u128 {
    u128::from_be_bytes(md5::compute(message).0)
}

#[test]
fn test_hashcash_size() {
    let input = MD5HashCashInput {
        complexity: 10,
        message: "Hello World".to_string(),
    };
    let output = hashcash(&input);
    assert_eq!(output.hashcode.len(), 32);
    let seed_hex = format!("{:x}", output.seed).to_string().to_uppercase();
    assert_eq!(seed_hex.len(), 16);
}

#[test]
fn test_compute_complexity() {
    assert_eq!(compute_complexity(compute_md5_to_u128("000000000000034Chello".to_string())), 9);
}