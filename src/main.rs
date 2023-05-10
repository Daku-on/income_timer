use std::io;
use std::time::{Duration, Instant};
use std::thread::sleep;

fn main() {
    println!("年収を入力してください（円）: ");

    let mut annual_income = String::new();
    io::stdin().read_line(&mut annual_income)
        .expect("Failed to read line");

    let annual_income: f64 = match annual_income.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let working_days_per_year = 120.0;
    let working_hours_per_day = 8.0;
    let minutes_per_hour = 60.0;

    // Calculate income per minute
    let income_per_minute = annual_income / (working_days_per_year * working_hours_per_day * minutes_per_hour);

    println!("あなたの1分あたりの収入は {:.2} 円です", income_per_minute);

    let start_time = Instant::now();
    println!("現在の総収入: 0円");

    loop {
        // Wait for a minute
        sleep(Duration::from_secs(60));

        let elapsed_minutes = start_time.elapsed().as_secs() / 60;
        let total_income = income_per_minute * elapsed_minutes as f64;

        println!("現在の総収入: {:.2} 円", total_income);

        // Check if more than 8 hours have passed
        if start_time.elapsed().as_secs() > (8 * 60 * 60) {
            println!("8時間経過しました。");
            break;
        }
    }
}