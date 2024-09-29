use std::fmt::Write;

use proc_macro::TokenStream;
use ruint::aliases::U256;
use sha2::{Digest, Sha256};
use veedo_ff::{FieldElement, PRIME};

const N_STATE: u64 = 2;
const N_COLS: u64 = 10;
const LENGTH: u64 = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CubeRootOperation {
    Square,
    Multiplication,
}

#[proc_macro]
pub fn round_constants(_input: TokenStream) -> TokenStream {
    let mut output = String::new();
    write_round_constants(&mut output).unwrap();
    output.parse().unwrap()
}

#[proc_macro]
pub fn mds_matrix(_input: TokenStream) -> TokenStream {
    let mut output = String::new();
    write_mds_matrix(&mut output).unwrap();
    output.parse().unwrap()
}

#[proc_macro]
pub fn fn_cube_root(_input: TokenStream) -> TokenStream {
    let mut output = String::new();
    write_fn_cube_root(&mut output).unwrap();
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
        let x_raw = constant.0 .0 .0;
        let y_raw = constant.1 .0 .0;

        writeln!(buf, "    [")?;
        writeln!(
            buf,
            "        ::veedo_ff::FieldElement::new_unchecked(::veedo_ff::BigInteger128::new({:?})),",
            x_raw
        )?;
        writeln!(
            buf,
            "        ::veedo_ff::FieldElement::new_unchecked(::veedo_ff::BigInteger128::new({:?})),",
            y_raw
        )?;
        writeln!(buf, "    ],")?;
    }

    writeln!(buf, "];")?;

    Ok(())
}

fn write_mds_matrix(buf: &mut String) -> std::fmt::Result {
    writeln!(
        buf,
        "pub const MDS_MATRIX: [[::veedo_ff::FieldElement; 2]; 2] = ["
    )?;

    let vectors = [
        [
            FieldElement::from(0x9adea15e459e2a62c3166a2a2054c3d_u128),
            FieldElement::from(0x187ccb0e2b63d835f7cf33d7555ca95d_u128),
        ],
        [
            FieldElement::from(0x1da2b56d14370ab50833f82f3966c9d7_u128),
            FieldElement::from(0x207e36a32b1da58011ae276251516aa4_u128),
        ],
    ];

    for vector in vectors {
        let x_raw = vector[0].0 .0;
        let y_raw = vector[1].0 .0;

        writeln!(buf, "    [")?;
        writeln!(
            buf,
            "        ::veedo_ff::FieldElement::new_unchecked(::veedo_ff::BigInteger128::new({:?})),",
            x_raw
        )?;
        writeln!(
            buf,
            "        ::veedo_ff::FieldElement::new_unchecked(::veedo_ff::BigInteger128::new({:?})),",
            y_raw
        )?;
        writeln!(buf, "    ],")?;
    }

    writeln!(buf, "];")?;

    Ok(())
}

#[allow(clippy::assertions_on_constants)]
fn write_fn_cube_root(buf: &mut String) -> std::fmt::Result {
    const POW: u128 = (2 * PRIME - 1) / 3;
    assert!(POW != 0, "exponent must not be zero");

    const POW_LIMBS: [u64; 2] = [POW as u64, (POW >> 64) as u64];

    let mut ops = vec![];
    let mut square_count = 0;
    let mut multiply_count = 0;

    for bit in veedo_ff::BitIteratorBE::without_leading_zeros(POW_LIMBS).skip(1) {
        ops.push(CubeRootOperation::Square);
        square_count += 1;
        if bit {
            ops.push(CubeRootOperation::Multiplication);
            multiply_count += 1;
        }
    }

    writeln!(buf, "#[inline(always)]")?;
    writeln!(buf, "/// Finds cube root of a field element.")?;
    writeln!(buf, "///")?;
    writeln!(
        buf,
        "/// The function is unrolled at compile time to save the runtime iteration \
        over exponent bits. It contains {} squaring and {} multiplication operations.",
        square_count, multiply_count
    )?;
    writeln!(
        buf,
        "pub fn cube_root(x: &::veedo_ff::FieldElement) -> ::veedo_ff::FieldElement {{"
    )?;

    // This optimization is possible due to non-zero exp.
    writeln!(buf, "    let mut res = *x;")?;

    for op in ops {
        match op {
            CubeRootOperation::Square => writeln!(buf, "    res.square_in_place();")?,
            CubeRootOperation::Multiplication => writeln!(buf, "    res *= x;")?,
        }
    }

    writeln!(buf, "    res")?;
    writeln!(buf, "}}")?;

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
        .map(|chunk| (FieldElement::from(chunk[0]), FieldElement::from(chunk[1])))
        .collect()
}
