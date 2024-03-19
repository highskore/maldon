use std::{ error, fmt, ops::Deref, str::FromStr };

use clap::Parser;

use alloy_primitives::Address;

/// Pattern.
#[derive(Debug, Clone)]
pub struct Pattern(Box<str>);

impl Pattern {}

impl Deref for Pattern {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Pattern errors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatternError {
    /// The pattern's length exceeds 39 characters or the pattern is empty.
    InvalidPatternLength,
    /// The patters is not in hexadecimal format.
    NonHexPattern,
}

impl fmt::Display for PatternError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatternError::InvalidPatternLength => write!(f, "invalid length"),
            PatternError::NonHexPattern => write!(f, "pattern must be hex digits only"),
        }
    }
}

impl error::Error for PatternError {}

impl FromStr for Pattern {
    type Err = PatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() >= 40 || s.is_empty() {
            return Err(PatternError::InvalidPatternLength);
        }

        if s.chars().any(|c| !c.is_ascii_hexdigit()) {
            return Err(PatternError::NonHexPattern);
        }

        Ok(Self(s.into()))
    }
}

#[derive(Parser, Debug, Clone)]
#[command(name = "maldon")]
#[command(about = "Maldon is a fast CREATE2 and CREATE3 salt miner.")]
pub enum Maldon {
    /// Mines a CREATE3 salt.
    Create3 {
        /// Address of the contract deployer.
        deployer: Address,
        /// Address of the Factory contract. Defaults to the Aori Factory.
        #[clap(short, long)]
        factory: Option<Address>,
        zero_count: usize,
    },
}
