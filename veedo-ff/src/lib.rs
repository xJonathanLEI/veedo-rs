pub use ff::{Field, PrimeField};

pub const PRIME: u128 = 0x30000003000000010000000000000001;

#[derive(PrimeField)]
#[PrimeFieldModulus = "63802944035360449460622495747797942273"]
#[PrimeFieldGenerator = "7"]
#[PrimeFieldReprEndianness = "little"]
pub struct FieldElement([u64; 2]);

impl FieldElement {
    pub const fn new(repr: [u64; 2]) -> Self {
        Self(repr)
    }

    pub const fn repr(&self) -> [u64; 2] {
        self.0
    }
}
