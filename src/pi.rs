use num_bigint::ToBigInt;

/// Calculates Pi to a specific number of decimal places for dynamic expansion.
pub fn calculate_pi_to_precision(digits: usize) -> String {
    let mut q = 1.to_bigint().unwrap();
    let mut r = 0.to_bigint().unwrap();
    let mut t = 1.to_bigint().unwrap();
    let mut k = 1.to_bigint().unwrap();
    let mut n = 3.to_bigint().unwrap();
    let mut l = 3.to_bigint().unwrap();

    let mut result = String::new();
    let mut count = 0;

    // Standard digit-by-digit spigot-style algorithm for stability
    while count < digits {
        if &q * 4 + &r - &t < &n * &t {
            result.push_str(&n.to_string());
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
    result
}
