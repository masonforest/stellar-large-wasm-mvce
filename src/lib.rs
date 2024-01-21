
#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn run(_env: Env, index: u32) -> u32 {
        include_bytes!("70kb.bin")[index as usize].into()
    }
}