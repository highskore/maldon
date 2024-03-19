mod cli;
mod mine;

use clap::Parser;

use cli::Maldon;

use mine::{ Create3Miner, Miner };

const CREATE3_DEFAULT_FACTORY: [u8; 20] = [
    0x2d, 0xfc, 0xc7, 0x41, 0x5d, 0x89, 0xaf, 0x82, 0x8c, 0xbe, 0xf0, 0x05, 0xf0, 0xd0, 0x72, 0xd8,
    0xb3, 0xf2, 0x31, 0x83,
];

fn main() {
    let (address, salt, zero_count) = match Maldon::parse() {
        Maldon::Create3 { deployer, factory, zero_count } => {
            let factory = if let Some(factory) = factory {
                factory
            } else {
                CREATE3_DEFAULT_FACTORY.into()
            };

            Create3Miner::new(factory, deployer).mine(zero_count)
        }
    };

    println!("Found salt {salt:?} ==> {address:?} with {zero_count} zero bytes.");
}
