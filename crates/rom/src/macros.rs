#![allow(unused_macros)]

/// Asserts that a condition is true at compile time.
macro_rules! static_assert {
    ($cond:expr $(,)?) => {
        const _: [(); 0 - !{
            const COND: bool = $cond;
            COND
        } as usize] = [];
    };
}
