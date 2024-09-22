use ark_ff::fields::{Fp128, MontBackend, MontConfig};

pub use ark_ff::{BigInteger128, BitIteratorBE, Field};

pub const PRIME: u128 = 0x30000003000000010000000000000001;

#[derive(MontConfig)]
#[modulus = "63802944035360449460622495747797942273"]
#[generator = "3"]
pub struct FrConfig;

pub type FieldElement = Fp128<MontBackend<FrConfig, 2>>;
