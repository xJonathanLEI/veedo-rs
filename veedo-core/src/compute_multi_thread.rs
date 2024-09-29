use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use veedo_ff::{BigInteger128, FieldElement};

use crate::{
    constants::{MDS_MATRIX, ROUND_CONSTANTS},
    utils::cube_root,
};

/// The computation thread state that's shared between the 2 worker threads.
///
/// Two slots are used as it's possible for one thread to have the next cube root ready before the
/// other thread has even consumed the current root. With just one slot, the current thread would
/// then have to check whether the existing value has been consumed by the other thread or not,
/// adding overhead to the synchronization.
///
/// Having more than 2 slots would not add any performance benefit, as it's impossible for one
/// thread to get ahead of the other by more than one slot due to the matrix multiplication step.
#[derive(Debug, Clone)]
struct SyncState<'a> {
    even_root: &'a AtomicFieldElement,
    odd_root: &'a AtomicFieldElement,
    count: &'a AtomicUsize,
}

/// A [`FieldElement`] value backed by two [`AtomicU64`] fields to be used in the thread
/// synchronization context.
#[derive(Debug, Default)]
struct AtomicFieldElement {
    low: AtomicU64,
    high: AtomicU64,
}

pub fn compute_delay_function(n_iters: usize, x: u128, y: u128) -> (FieldElement, FieldElement) {
    let x_montgomery = FieldElement::from(x);
    let y_montgomery = FieldElement::from(y);

    let x_state = SyncState {
        even_root: &AtomicFieldElement::default(),
        odd_root: &AtomicFieldElement::default(),
        count: &AtomicUsize::new(0),
    };
    let y_state = SyncState {
        even_root: &AtomicFieldElement::default(),
        odd_root: &AtomicFieldElement::default(),
        count: &AtomicUsize::new(0),
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

    let mut count = 0;

    for round_constant in ROUND_CONSTANTS.iter().cycle().take(n_iters) {
        // Compute cube root. This is the most expensive part of the iteration.
        current = cube_root(&current);

        count += 1;
        let is_even = count % 2 == 0;

        // There's no need to make sure the other thread has already read the slot, as it's simply
        // impossible for the current thread to be 2 slots ahead of the other.
        let current_root = if is_even {
            current_state.even_root
        } else {
            current_state.odd_root
        };
        current_root.low.store(current.0 .0[0], Ordering::Release);
        current_root.high.store(current.0 .0[1], Ordering::Release);
        current_state.count.store(count, Ordering::Release);

        // Finish half of the matrix multiplication and constant addition steps which do not depend
        // on the other thread.
        current = current * MDS_MATRIX[SLOT][SLOT] + round_constant[SLOT];

        // Wait for the state value from the other thread
        let other = {
            loop {
                if other_state.count.load(Ordering::Acquire) >= count {
                    break;
                }
            }

            let other_root = if is_even {
                other_state.even_root
            } else {
                other_state.odd_root
            };

            FieldElement::new_unchecked(BigInteger128::new([
                other_root.low.load(Ordering::Acquire),
                other_root.high.load(Ordering::Acquire),
            ]))
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
