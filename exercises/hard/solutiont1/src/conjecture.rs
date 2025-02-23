pub fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn is_composite(n: i32) -> bool {
    if n <= 3 {
        return false;
    }
    !is_prime(n)
}

//gemini2.1 think
pub fn goldbach_conjecture_1() -> String {
    let mut counterexamples = Vec::new();
    let mut num = 9;
    while counterexamples.len() < 2 {
        if is_composite(num) {
            let mut found = false;
            let mut y = 1;
            loop {
                let two_y_sq = 2 * y * y;
                if two_y_sq >= num {
                    break;
                }
                let p = num - two_y_sq;
                if is_prime(p) {
                    found = true;
                    break;
                }
                y += 1;
            }
            if !found {
                counterexamples.push(num);
            }
        }
        num += 2;
    }
    format!("{},{}", counterexamples[0], counterexamples[1])
}


//chatgpt
pub fn goldbach_conjecture() -> String {
    // 判断 n 是否为素数
    fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        for i in 2..=((n as f64).sqrt() as u64) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    let mut results = Vec::new();
    // 从 9 开始，只考虑奇数
    let mut n = 9;
    while results.len() < 2 {
        // 仅考虑奇合数（非素数）
        if !is_prime(n) {
            let mut can_be_expressed = false;
            // 枚举 k，从 1 开始，条件是 2*k^2 < n
            let mut k = 1;
            while 2 * k * k < n {
                // 如果 n - 2*k^2 是素数，则 n 可以表示成“素数 + 2×k^2”
                if is_prime(n - 2 * k * k) {
                    can_be_expressed = true;
                    break;
                }
                k += 1;
            }
            if !can_be_expressed {
                results.push(n);
            }
        }
        n += 2; // 只检查奇数
    }
    // 结果用字符串拼接，第一个数,第二个数
    format!("{},{}", results[0], results[1])
}


//grok
pub fn goldbach_conjecture_grok() -> String {
    // 用于存储满足条件的奇合数
    let mut counterexamples = Vec::new();
    // 从最小的奇合数9开始
    let mut n = 9;

    // 循环直到找到两个目标数
    while counterexamples.len() < 2 {
        // 检查n是否为合数（非素数）
        if !is_prime_grok(n) {
            let mut found = false;
            let mut k = 1;
            // 检查所有可能的k，使得2*k*k < n
            while 2 * k * k < n {
                let m = n - 2 * k * k;
                // 如果m > 1且为素数，则n可以表示为p + 2k^2
                if m > 1 && is_prime(m) {
                    found = true;
                    break;
                }
                k += 1;
            }
            // 如果没有找到合适的k，n是目标数
            if !found {
                counterexamples.push(n);
            }
        }
        // 跳到下一个奇数
        n += 2;
    }

    // 格式化输出为"第一个数,第二个数"
    format!("{},{}", counterexamples[0], counterexamples[1])
}

// 判断一个数是否为素数的辅助函数
fn is_prime_grok(m: i32) -> bool {
    if m <= 1 {
        return false;
    }
    if m == 2 {
        return true;
    }
    if m % 2 == 0 {
        return false;
    }
    // 只检查奇数除数直到sqrt(m)
    let mut i = 3;
    while i * i <= m {
        if m % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}


// // Miller-Rabin 素性测试
// fn pow_mod(a: u64, b: u64, mod: u64) -> u64 {
//     let mut result = 1;
//     a %= mod;
//     while b > 0 {
//         if b % 2 == 1 {
//             result = (result * a) % mod;
//         }
//         a = (a * a) % mod;
//         b /= 2;
//     }
//     result
// }

// fn is_prime(n: u64) -> bool {
//     if n <= 1 {
//         return false;
//     }
//     let mut d = n - 1;
//     let mut s = 0;
//     while d % 2 == 0 {
//         d /= 2;
//         s += 1;
//     }
//     // 测试基底，足够覆盖n < 2^64的情况
//     let bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
//     for a in bases.iter() {
//         if a >= n {
//             continue;
//         }
//         let x = pow_mod(*a, d, n);
//         if x == 1 || x == n - 1 {
//             continue;
//         }
//         for _ in 0..s-1 {
//             x = pow_mod(x, 2, n);
//             if x == n - 1 {
//                 break;
//             }
//         }
//         if x != n -1 {
//             return false;
//         }
//     }
//     true
// }

// // 判断是否为奇合数
// fn is_composite(n: u64) -> bool {
//     if n % 2 == 0 {
//         return false;
//     }
//     if n <= 1 {
//         return false;
//     }
//     let sqrt_n = math::sqrt(n) as u64;
//     for i in (3..=sqrt_n).step_by(2) {
//         if n % i == 0 {
//             return true;
//         }
//     }
//     false
// }

// // 检查n是否能表示为素数p + 2*q^2
// fn check(n: u64) -> bool {
//     let max_q = ((n - 2) / 2).floor().sqrt() as u64;
//     for q in 0..=max_q {
//         let two_q_sq = 2 * q * q;
//         if two_q_sq > n {
//             break;
//         }
//         let p = n - two_q_sq;
//         if p < 2 {
//             continue;
//         }
//         if is_prime(p.try_into().unwrap()) {
//             return true;
//         }
//     }
//     false
// }

// pub fn goldbach_conjecture() -> String {
//     let mut found = 0;
//     let mut first = None;
//     let mut second = None;
//     let mut current = 9; // 第一个奇合数是9

//     while found < 2 {
//         if is_composite(current) {
//             if !check(current.try_into().unwrap()) {
//                 match found {
//                     0 => first = Some(current),
//                     1 => second = Some(current),
//                     _ => break,
//                 }
//                 found += 1;
//             }
//         }
//         current += 2;

//         // 防止无限循环，可根据实际情况调整上限
//         if current > 1_000_000 {
//             break;
//         }
//     }

//     format!("{},{}", first.unwrap(), second.unwrap())
// }