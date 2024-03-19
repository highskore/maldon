use rayon::prelude::{ IntoParallelIterator, ParallelIterator };

use rand::{ thread_rng, Rng };

use alloy_primitives::{ keccak256, Address, FixedBytes };

pub trait Miner {
    /// Runs the Miner.
    fn mine(&self, zero_count: usize);
}

#[derive(Debug, Clone, Copy)]
pub struct Create3Miner {
    factory: Address,
    deployer: Address,
}

impl Create3Miner {
    const PROXY_INIT_CODE_HASH: [u8; 32] = [
        0x21, 0xc3, 0x5d, 0xbe, 0x1b, 0x34, 0x4a, 0x24, 0x88, 0xcf, 0x33, 0x21, 0xd6, 0xce, 0x54, 0x2f,
        0x8e, 0x9f, 0x30, 0x55, 0x44, 0xff, 0x09, 0xe4, 0x99, 0x3a, 0x62, 0x31, 0x9a, 0x49, 0x7c, 0x1f,
    ];
    pub fn new(factory: Address, deployer: Address) -> Self {
        Self { factory, deployer }
    }
}

impl Miner for Create3Miner {
    fn mine(&self, zero_count: usize) {
        let mut rng = thread_rng();

        let mut salt_buffer = [0u8; 52];
        salt_buffer[0..20].copy_from_slice(self.deployer.as_slice());

        let mut proxy_create_buffer = [0u8; 23];
        proxy_create_buffer[0..2].copy_from_slice(&[0xd6, 0x94]);
        proxy_create_buffer[22] = 0x01;

        loop {
            let mut salt: FixedBytes<32> = FixedBytes::default();
            rng.fill(&mut salt[16..]);

            (0..u128::MAX).into_par_iter().for_each(move |nonce| {
                let mut salt = salt;
                salt[0..16].copy_from_slice(&nonce.to_be_bytes());
                let mut salt_buffer = salt_buffer;
                salt_buffer[20..].copy_from_slice(salt.as_slice());
                let mut proxy_create_buffer = proxy_create_buffer;
                let proxy = self.factory.create2(
                    keccak256(salt_buffer),
                    Self::PROXY_INIT_CODE_HASH
                );
                proxy_create_buffer[2..22].copy_from_slice(proxy.as_slice());
                let hash = keccak256(proxy_create_buffer);

                // Convert each byte in the Ethereum address portion of the hash into a hexadecimal string
                let address_hex: String = hash[12..32]
                    .iter()
                    .map(|byte| format!("{:02x}", byte))
                    .collect();

                // Count '0' characters in the hexadecimal string
                let zero_char_count = address_hex
                    .chars()
                    .filter(|&c| c == '0')
                    .count();

                if zero_char_count >= zero_count {
                    // If conditions met, return the found address and salt
                    println!(
                        "Found: Address: {:?}, Salt: {salt:?}, Zero count: {zero_char_count:?}",
                        Address::from_slice(&hash[12..32])
                    );
                }
            });
        }
    }
}
