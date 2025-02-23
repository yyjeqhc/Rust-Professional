use std::num::ParseIntError;

    // 将 num_str 解析为源进制，并返回十进制数值
    pub fn parse_number(num_str: &str) -> Result<(u32, u32), ParseIntError> {
        let idx = num_str.find('(').unwrap();
        let num_part = &num_str[..idx];
        let base_part = &num_str[idx + 1..num_str.len() - 1];  // 去掉括号
        let num = u32::from_str_radix(num_part, 10)?; // 先转成 10 进制
        let base = base_part.parse::<u32>()?; // 读取进制部分
        Ok((num, base))
    }

    // 将十进制数转换为目标进制
    pub fn to_base(num: u32, base: u32) -> String {
        let mut result = String::new();
        let mut num = num;
        
        while num > 0 {
            let digit = num % base;
            num /= base;
            result.push_str(&format!("{:x}", digit)); // 使用 x 格式输出 16 进制字符（默认小写）
        }
        
        if result.is_empty() {
            result.push('0'); // 如果结果为空，说明是 0
        }

        result.chars().rev().collect() // 反转字符串，得到正确的顺序
    }

    // 综合调用的函数：将任意进制的数字转换为目标进制
    pub fn convert_base(num_str: &str, base_to: u32) -> String {
        match parse_number(num_str) {
            Ok((num, base_from)) => {
                let decimal_value = u32::from_str_radix(&num_str[..num_str.find('(').unwrap()], base_from).unwrap();
                to_base(decimal_value, base_to)
            }
            Err(_) => String::from("Invalid input"),
        }
    }

