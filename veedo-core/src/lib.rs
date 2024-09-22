use veedo_ff::{Field, FieldElement};

mod constants;
use constants::{MDS_MATRIX, ROUND_CONSTANTS};

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

veedo_codegen::fn_cube_root!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_delay_function() {
        let res = compute_delay_function(40960, 1, 1);

        assert_eq!(
            res.0,
            FieldElement::from(0xbb0f72c981ff840c10857b5af871006_u128),
            "Wrong x value"
        );
        assert_eq!(
            res.1,
            FieldElement::from(0xefccc63f8534a514165c22a32bf0911_u128),
            "Wrong y value"
        );
    }

    #[test]
    fn test_compute_delay_function_with_initial_state() {
        let res = compute_delay_function(
            20480,
            0x28d5eafa6da4b52d946184fc5cb5aa8a_u128,
            0xeacfd8b416443d9a3f8e8d7c4b4f492_u128,
        );

        assert_eq!(
            res.0,
            FieldElement::from(0x17ce4ec2dcdd566b7249175175a0e77a_u128),
            "Wrong x value"
        );
        assert_eq!(
            res.1,
            FieldElement::from(0x18ef1f3b87b66c371199d2e41982c9a_u128),
            "Wrong y value"
        );
    }
}
