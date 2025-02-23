pub fn dp_rec_mc(amount: u32) -> u32 {
    let coins = [1, 2, 5, 10, 20, 30, 50, 100]; // 定义纸币面额
    let mut dp = vec![u32::MAX; (amount + 1) as usize]; // 初始化动态规划数组，设为最大值
    dp[0] = 0; // 金额为0时，纸币数量为0

    for i in 1..=amount {
        for &coin in &coins {
            if i >= coin {
                dp[i as usize] = dp[i as usize].min(dp[(i - coin) as usize] + 1);
            }
        }
    }

    dp[amount as usize]
}
