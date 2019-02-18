//! Set of some useful functions

use std::time::{Duration, Instant};

/// Measure the execution time of the given closure
#[allow(dead_code)]
pub fn measure_time<T, F: FnOnce() -> T>(f: F) -> (Duration, T) {
    let now = Instant::now();
    let result = f();
    (now.elapsed(), result)
}

