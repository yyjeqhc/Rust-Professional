pub fn find_max_prime_factor(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    let mut factors = Vec::new();
    factorize(n, &mut factors);
    factors.into_iter().max().unwrap_or(1)
}

fn factorize(n: u128, factors: &mut Vec<u128>) {
    if n == 1 {
        return;
    }

    // Check for small primes 2, 3, 5
    for &p in [2, 3, 5].iter() {
        if n % p == 0 {
            factors.push(p);
            let mut m = n / p;
            while m % p == 0 {
                m /= p;
            }
            factorize(m, factors);
            return;
        }
    }

    // Check other factors up to sqrt(n) or a threshold
    let threshold = 1_000_000;
    let sqrt_n = (n as f64).sqrt() as u128;
    let max_i = std::cmp::min(threshold, sqrt_n);
    let mut i = 7;
    let mut di = 4; // Alternating steps of 2 and 4 for 6k ± 1
    while i <= max_i {
        if n % i == 0 {
            factors.push(i);
            let mut m = n / i;
            while m % i == 0 {
                m /= i;
            }
            factorize(m, factors);
            return;
        }
        i += di;
        di = 6 - di; // Toggle between 2 and 4
    }

    // Remaining n is a prime or needs factorization
    if is_prime(n) {
        factors.push(n);
    } else {
        let d = pollards_rho(n);
        factorize(d, factors);
        factorize(n / d, factors);
    }
}

fn is_prime(n: u128) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    let bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &a in &bases {
        if a >= n {
            continue;
        }
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut composite = true;
        for _ in 0..s - 1 {
            x = mod_pow(x, 2, n);
            if x == n - 1 {
                composite = false;
                break;
            }
        }
        if composite {
            return false;
        }
    }
    true
}

fn mod_pow(mut base: u128, mut exponent: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent >>= 1;
        base = (base * base) % modulus;
    }
    result
}

fn pollards_rho(n: u128) -> u128 {
    if n == 1 {
        return 1;
    }
    if n % 2 == 0 {
        return 2;
    }
    if n % 3 == 0 {
        return 3;
    }
    if n % 5 == 0 {
        return 5;
    }

    let mut c = 1;
    loop {
        let f = |x: u128| (mod_pow(x, 2, n) + c )% n;
        let mut x = 2;
        let mut y = 2;
        let mut d = 1;

        while d == 1 {
            x = f(x);
            y = f(f(y));
            d = gcd(x.abs_diff(y), n);
        }

        if d != n {
            return d;
        }
        c += 1;
        if c > 5 {
            c = 1;
        }
    }
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}