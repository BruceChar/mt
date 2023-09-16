//! Cli and web service of math tools like `GCD`.
//! Support:
//!     1. any number of integers
//!     2. concurrency
//!
//!
//!

pub fn gcd(
    mut a: u64,
    mut b: u64,
) -> u64 {
    assert!(a != 0 && b != 0);
    while a != 0 {
        if a < b {
            let t =
                a;
            a = b;
            b = t;
        }
        a %= b;
    }
    b
}

pub fn gcd_iter(
    nums: Vec<u64>,
) -> u64 {
    assert!(!nums
        .is_empty(
        ));
    if nums.len()
        == 1
    {
        return nums[0];
    }
    let mut a =
        nums[0];
    for b in nums
        .iter()
        .skip(1)
    {
        a = gcd(
            a, *b,
        );
    }
    a
}
