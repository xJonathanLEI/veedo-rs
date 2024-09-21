use std::fmt::Write;

use proc_macro::TokenStream;
use ruint::aliases::U256;
use sha2::{Digest, Sha256};
use veedo_ff::{FieldElement, PrimeField, PRIME};

const N_STATE: u64 = 2;
const N_COLS: u64 = 10;
const LENGTH: u64 = 256;

#[proc_macro]
pub fn round_constants(_input: TokenStream) -> TokenStream {
    let mut output = String::new();
    write_round_constants(&mut output).unwrap();
    output.parse().unwrap()
}

#[proc_macro]
pub fn mutplication_matrix(_input: TokenStream) -> TokenStream {
    let mut output = String::new();
    write_mutplication_matrix(&mut output).unwrap();
    output.parse().unwrap()
}

fn write_round_constants(buf: &mut String) -> std::fmt::Result {
    let constants = generate_constants();

    writeln!(
        buf,
        "pub const ROUND_CONSTANTS: [[::veedo_ff::FieldElement; 2]; {}] = [",
        constants.len()
    )?;

    for constant in constants {
        let x_raw = constant.0.repr();
        let y_raw = constant.1.repr();

        writeln!(buf, "    [")?;
        writeln!(buf, "        ::veedo_ff::FieldElement::new({:?}),", x_raw)?;
        writeln!(buf, "        ::veedo_ff::FieldElement::new({:?}),", y_raw)?;
        writeln!(buf, "    ],")?;
    }

    writeln!(buf, "];")?;

    Ok(())
}

fn write_mutplication_matrix(buf: &mut String) -> std::fmt::Result {
    writeln!(
        buf,
        "pub const MDS_MATRIX: [[::veedo_ff::FieldElement; 2]; 2] = ["
    )?;

    let vectors = [
        [
            FieldElement::from_u128(0x9adea15e459e2a62c3166a2a2054c3d_u128),
            FieldElement::from_u128(0x187ccb0e2b63d835f7cf33d7555ca95d_u128),
        ],
        [
            FieldElement::from_u128(0x1da2b56d14370ab50833f82f3966c9d7_u128),
            FieldElement::from_u128(0x207e36a32b1da58011ae276251516aa4_u128),
        ],
    ];

    for vector in vectors {
        let x_raw = vector[0].repr();
        let y_raw = vector[1].repr();

        writeln!(buf, "    [")?;
        writeln!(buf, "        ::veedo_ff::FieldElement::new({:?}),", x_raw)?;
        writeln!(buf, "        ::veedo_ff::FieldElement::new({:?}),", y_raw)?;
        writeln!(buf, "    ],")?;
    }

    writeln!(buf, "];")?;

    Ok(())
}

fn generate_constants() -> Vec<(FieldElement, FieldElement)> {
    let mut constant_seq = vec![];

    for idx in 0..LENGTH {
        for col in 0..N_COLS {
            for state in 0..N_STATE {
                let mut hasher = Sha256::new();
                hasher.update(format!("Veedo_{}_{}_{}", state, col, idx));
                let hash: [u8; 32] = hasher.finalize().into();

                let item = U256::from_be_bytes(hash);
                let item: u128 = (item % U256::from(PRIME)).try_into().unwrap();

                constant_seq.push(item);
            }
        }
    }

    constant_seq
        .chunks_exact(2)
        .map(|chunk| {
            (
                FieldElement::from_u128(chunk[0]),
                FieldElement::from_u128(chunk[1]),
            )
        })
        .collect()
}
