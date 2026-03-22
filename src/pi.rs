use num_bigint::{BigInt, ToBigInt};
use num_traits::{One, Zero};

pub fn calculate_pi_to_precision(digits: usize) -> String {
    // Chudnovsky Algorithm Implementation in Rust
    let decimal_precision = digits + 2;
    let iterations = (digits / 14) + 1;

    let mut q = 1.to_bigint().unwrap();
    let mut r = 0.to_bigint().unwrap();
    let mut t = 1.to_bigint().unwrap();
    let mut k = 1.to_bigint().unwrap();
    let mut n = 3.to_bigint().unwrap();
    let mut l = 3.to_bigint().unwrap();

    let mut result = String::new();
    let mut count = 0;

    while count < digits {
        if &q * 4 + &r - &t < &n * &t {
            result.push_str(&n.to_string());
            if count == 0 { result.push('.'); }
            let nr = (&r - &n * &t) * 10;
            n = (&q * 3 + &r) * 10 / &t - &n * 10;
            q *= 10;
            r = nr;
            count += 1;
        } else {
            let nr = (&q * 2 + &r) * &l;
            let nn = (&q * &k * 7 + 2 + &r * &l) / (&t * &l);
            q *= &k;
            t *= &l;
            l += 2;
            k += 1;
            n = nn;
            r = nr;
        }
    }
    result.replace(".", "")
}
