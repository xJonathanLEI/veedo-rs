use veedo_ff::FieldElement;

use crate::constants::{MDS_MATRIX_INVERSE, ROUND_CONSTANTS};

#[cfg(target_arch = "wasm32")]
pub fn compute_delay_function(n_iters: usize, x: u128, y: u128) -> (FieldElement, FieldElement) {
    use crate::{constants::MDS_MATRIX, utils::cube_root};

    let (mut x, mut y) = (FieldElement::from(x), FieldElement::from(y));

    for round_constant in ROUND_CONSTANTS.iter().cycle().take(n_iters) {
        // Compute cube root.
        (x, y) = (cube_root(&x), cube_root(&y));

        // Multiply with the matrix and add constants.
        (x, y) = (
            x * MDS_MATRIX[0][0] + y * MDS_MATRIX[0][1] + round_constant[0],
            x * MDS_MATRIX[1][0] + y * MDS_MATRIX[1][1] + round_constant[1],
        );
    }

    (x, y)
}

pub fn inverse_delay_function(n_iters: usize, x: u128, y: u128) -> (FieldElement, FieldElement) {
    let (mut x, mut y) = (FieldElement::from(x), FieldElement::from(y));

    for round_constant in ROUND_CONSTANTS
        .iter()
        .rev()
        .cycle()
        .skip(ROUND_CONSTANTS.len() - (n_iters % ROUND_CONSTANTS.len()))
        .take(n_iters)
    {
        // Undo round constant addition.
        (x, y) = (x - round_constant[0], y - round_constant[1]);

        // Undo matrix multiplication.
        (x, y) = (
            x * MDS_MATRIX_INVERSE[0][0] + y * MDS_MATRIX_INVERSE[0][1],
            x * MDS_MATRIX_INVERSE[1][0] + y * MDS_MATRIX_INVERSE[1][1],
        );

        // Undo cube root.
        (x, y) = (x * x * x, y * y * y);
    }

    (x, y)
}
