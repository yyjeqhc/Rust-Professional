pub fn new_birthday_probability(n: u32) -> f64 {
    if n > 365 {
        return 1.0; // 如果人数超过 365，则一定有重复生日
    }

    let mut prob_no_match = 1.0;
    for i in 0..n {
        prob_no_match *= (365 - i) as f64 / 365.0;
    }

    1.0 - prob_no_match
}