mod constants;
mod utils;

#[cfg(not(target_arch = "wasm32"))]
mod compute_multi_thread;
mod compute_single_thread;

#[cfg(not(target_arch = "wasm32"))]
pub use compute_multi_thread::compute_delay_function;
#[cfg(target_arch = "wasm32")]
pub use compute_single_thread::compute_delay_function;

// The inverse function is so fast that with a state size of 2, the threading overhead exceeds the
// benefit from parallelization. Therefore, the single-threaded implementation is always used even
// when threading is available.
pub use compute_single_thread::inverse_delay_function;

#[cfg(test)]
mod tests {
    use veedo_ff::FieldElement;

    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
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
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
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

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_inverse_delay_function() {
        let res = inverse_delay_function(
            40960,
            0xbb0f72c981ff840c10857b5af871006_u128,
            0xefccc63f8534a514165c22a32bf0911_u128,
        );

        assert_eq!(res.0, FieldElement::from(1), "Wrong x value");
        assert_eq!(res.1, FieldElement::from(1), "Wrong y value");
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_inverse_delay_function_with_initial_state() {
        let res = inverse_delay_function(
            20480,
            0x17ce4ec2dcdd566b7249175175a0e77a_u128,
            0x18ef1f3b87b66c371199d2e41982c9a_u128,
        );

        assert_eq!(
            res.0,
            FieldElement::from(0x28d5eafa6da4b52d946184fc5cb5aa8a_u128),
            "Wrong x value"
        );
        assert_eq!(
            res.1,
            FieldElement::from(0xeacfd8b416443d9a3f8e8d7c4b4f492_u128),
            "Wrong y value"
        );
    }
}
