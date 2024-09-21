use veedo_ff::{Field, FieldElement, PrimeField, PRIME};

mod constants;
use constants::{MDS_MATRIX, ROUND_CONSTANTS};

const POW: u128 = (2 * PRIME - 1) / 3;

pub fn compute_delay_function(n_iters: usize, x: u128, y: u128) -> (FieldElement, FieldElement) {
    assert!(n_iters > 0, "n_iters should be a positive number");

    let (mut x, mut y) = (FieldElement::from_u128(x), FieldElement::from_u128(y));

    let pow = [POW as u64, (POW >> 64) as u64];

    for round_constant in ROUND_CONSTANTS.iter().cycle().take(n_iters) {
        // Cube root
        (x, y) = (x.pow_vartime(pow), y.pow_vartime(pow));

        // Multiply with the matrix and add constants
        (x, y) = (
            x * MDS_MATRIX[0][0] + y * MDS_MATRIX[0][1] + round_constant[0],
            x * MDS_MATRIX[1][0] + y * MDS_MATRIX[1][1] + round_constant[1],
        );
    }

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_delay_function() {
        let res = compute_delay_function(40960, 1, 1);

        assert_eq!(
            res.0,
            FieldElement::from_u128(0xbb0f72c981ff840c10857b5af871006_u128),
            "Wrong x value"
        );
        assert_eq!(
            res.1,
            FieldElement::from_u128(0xefccc63f8534a514165c22a32bf0911_u128),
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
            FieldElement::from_u128(0x17ce4ec2dcdd566b7249175175a0e77a_u128),
            "Wrong x value"
        );
        assert_eq!(
            res.1,
            FieldElement::from_u128(0x18ef1f3b87b66c371199d2e41982c9a_u128),
            "Wrong y value"
        );
    }
}
