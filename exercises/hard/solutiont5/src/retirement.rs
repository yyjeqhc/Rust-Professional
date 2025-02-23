//根据计算开始年份，实际出身日期，以及延迟退休间隔月数，计算延迟退休几个月。
// 根据计算开始年份，实际出生日期，以及延迟退休间隔月数，计算实际退休年月
fn handle(start_time: i32, time: &str, gap_month: i32, original_retire_age: i32) -> (i32, i32) {
    // 解析出生年月
    let parts: Vec<&str> = time.split('-').collect();
    let birth_year: i32 = parts[0].parse().unwrap();
    let birth_month: i32 = parts[1].parse().unwrap();

    // 计算原退休日期
    let mut retire_year = birth_year + original_retire_age;
    let mut retire_month = birth_month;

    // 如果出生年份早于起始年份，无延迟
    if birth_year < start_time {
        return (retire_year, retire_month);
    }

    // 计算从start_time到birth_time的总月数
    let total_months = (birth_year - start_time) * 12 + birth_month ;

    // 根据间隔计算延迟月数
    let delay_months = if total_months <= 0 {
        0
    } else {
        let increments = (total_months as f64 / gap_month as f64).ceil() as i32;
        increments // 结果已经向上取整
    };

    // 根据类型设置最大延迟
    let capped_delay = match gap_month {
        2 => delay_months.min(60), // 女50岁，最大延迟60个月
        4 => delay_months.min(36), // 男职工和女55岁，最大延迟36个月
        _ => delay_months,
    };

    // 计算实际退休日期
    let mut actual_retire_year = retire_year;
    let mut actual_retire_month = retire_month + capped_delay;
    while actual_retire_month > 12 {
        actual_retire_year += 1;
        actual_retire_month -= 12;
    }

    (actual_retire_year, actual_retire_month)
}

pub fn retire_time(time: &str, tp: &str) -> String {
    // 解析出生年月
    let parts: Vec<&str> = time.split('-').collect();
    let birth_year: i32 = parts[0].parse().unwrap();
    let birth_month: i32 = parts[1].parse().unwrap();

    // 根据人员类型调用 handle 并获取原退休年龄
    let (actual_retire_year, actual_retire_month, original_retire_age, delay_months) = match tp {
        "原法定退休年龄50周岁女职工" => {
            let (year, month) = handle(1975, time, 2, 50);
            let delay = (year - birth_year - 50) * 12 + (month - birth_month);
            (year, month, 50, delay)
        }
        "原法定退休年龄55周岁女职工" => {
            let (year, month) = handle(1970, time, 4, 55);
            let delay = (year - birth_year - 55) * 12 + (month - birth_month);
            (year, month, 55, delay)
        }
        "男职工" => {
            let (year, month) = handle(1965, time, 4, 60);
            let delay = (year - birth_year - 60) * 12 + (month - birth_month);
            (year, month, 60, delay)
        }
        _ => panic!("Invalid category"),
    };

    // 计算退休年龄（精确到2位小数）
    let years = (actual_retire_year - birth_year) as f64 + 
                (actual_retire_month - birth_month) as f64 / 12.0;
    // let retire_age = format!("{:.2}", years);
    let retire_age = if years.fract() == 0.0 {
        format!("{}", years as i32) // 去掉小数部分
    } else {
        format!("{:.2}", years) // 保留两位小数
    };

    // 格式化输出
    format!("{:04}-{:02},{},{}", actual_retire_year, actual_retire_month, retire_age, delay_months)
}