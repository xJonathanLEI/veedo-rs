use veedo_ff::FieldElement;

use crate::{
    constants::{MDS_MATRIX, ROUND_CONSTANTS},
    utils::cube_root,
};

pub fn compute_delay_function(n_iters: usize, x: u128, y: u128) -> (FieldElement, FieldElement) {
    let (mut x, mut y) = (FieldElement::from(x), FieldElement::from(y));

    for round_constant in ROUND_CONSTANTS.iter().cycle().take(n_iters) {
        // Cube root
        (x, y) = (cube_root(&x), cube_root(&y));

        // Multiply with the matrix and add constants
        (x, y) = (
            x * MDS_MATRIX[0][0] + y * MDS_MATRIX[0][1] + round_constant[0],
            x * MDS_MATRIX[1][0] + y * MDS_MATRIX[1][1] + round_constant[1],
        );
    }

    (x, y)
}
