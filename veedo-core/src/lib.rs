use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use veedo_ff::{BigInteger128, Field, FieldElement};

mod constants;
use constants::{MDS_MATRIX, ROUND_CONSTANTS};

#[derive(Debug, Clone)]
struct SyncState<'a> {
    low: &'a AtomicU64,
    high: &'a AtomicU64,
    ready: &'a AtomicBool,
}

pub fn compute_delay_function(n_iters: usize, x: u128, y: u128) -> (FieldElement, FieldElement) {
    let x_montgomery = FieldElement::from(x);
    let y_montgomery = FieldElement::from(y);

    let x_state = SyncState {
        low: &AtomicU64::new(0),
        high: &AtomicU64::new(0),
        ready: &AtomicBool::new(false),
    };
    let y_state = SyncState {
        low: &AtomicU64::new(0),
        high: &AtomicU64::new(0),
        ready: &AtomicBool::new(false),
    };

    std::thread::scope(|s| {
        let x_thread =
            s.spawn(|| compute_in_thread::<0>(n_iters, x_montgomery, &x_state, &y_state));
        let y_thread =
            s.spawn(|| compute_in_thread::<1>(n_iters, y_montgomery, &y_state, &x_state));

        // Joining never fails as the computation threads never panic.
        let x = unsafe { x_thread.join().unwrap_unchecked() };
        let y = unsafe { y_thread.join().unwrap_unchecked() };

        (x, y)
    })
}

#[inline(always)]
fn compute_in_thread<const SLOT: usize>(
    n_iters: usize,
    init_value: FieldElement,
    current_state: &SyncState,
    other_state: &SyncState,
) -> FieldElement {
    let mut current = init_value;

    for round_constant in ROUND_CONSTANTS.iter().cycle().take(n_iters) {
        // Compute cube root. This is the most expensive part of the iteration.
        current = cube_root(&current);

        // Make the current thread's state available to the other thread.
        loop {
            if !current_state.ready.load(Ordering::Acquire) {
                break;
            }
        }
        current_state.low.store(current.0 .0[0], Ordering::Release);
        current_state.high.store(current.0 .0[1], Ordering::Release);
        current_state.ready.store(true, Ordering::Release);

        // Finish half of the matrix multiplication and constant addition steps which do not depend
        // on the other thread.
        current = current * MDS_MATRIX[SLOT][SLOT] + round_constant[SLOT];

        // Wait for the state value from the other thread
        let other = {
            loop {
                if other_state.ready.load(Ordering::Acquire) {
                    break;
                }
            }

            let other_low = other_state.low.load(Ordering::Acquire);
            let other_high = other_state.high.load(Ordering::Acquire);
            other_state.ready.store(false, Ordering::Release);

            FieldElement::new_unchecked(BigInteger128::new([other_low, other_high]))
        };

        // Finish the round by completing the other half of the matrix multiplication.
        current += other
            * if SLOT == 0 {
                MDS_MATRIX[0][1]
            } else {
                MDS_MATRIX[1][0]
            };
    }

    current
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
