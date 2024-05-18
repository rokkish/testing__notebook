use clap::Parser;
use chrono::{NaiveDateTime, Weekday, prelude::*};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct MyArgs {
    #[arg(short, long)]
    day_of_week: String,

    #[arg(short, long)]
    time: String,

    #[arg(short, long)]
    membership: bool,
}

fn solve(
    day_of_week: String,
    time: String,
    membership: bool,
) -> i32 {
    let weekday = day_of_week.parse::<Weekday>().unwrap();
    let time = NaiveDateTime::parse_from_str(&time, "%Y-%m-%d %H:%M:%S").unwrap();
    let input_time = Utc.with_ymd_and_hms(time.year(), time.month(), time.day(), time.hour(), time.minute(), time.second()).unwrap();
    let early_time = Utc.with_ymd_and_hms(time.year(), time.month(), time.day(), 8, 45, 00).unwrap();
    let later_time = Utc.with_ymd_and_hms(time.year(), time.month(), time.day(), 18, 00, 00).unwrap();

    if membership {
        return 0;
    }

    if weekday.num_days_from_monday() <= 4
        && input_time >= early_time
            && input_time < later_time {
        return 0;
    }
    110
}

fn main() {
    let args = MyArgs::parse();
    println!("{:?}", solve(args.day_of_week, args.time, args.membership));
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Monday", "2024-05-13 00:00:00", false, 110)]
    #[case("Monday", "2024-05-13 08:44:59", false, 110)] // boundary
    #[case("Tuesday", "2024-05-14 08:45:00", false, 0)] // boundary
    #[case("Tuesday", "2024-05-14 12:00:00", false, 0)] // representive
    #[case("Tuesday", "2024-05-14 17:59:59", false, 0)] // boundary
    #[case("Wednesday", "2024-05-15 18:00:00", false, 110)] // boundary
    #[case("Saturday", "2024-05-18 08:45:00", false, 110)]
    #[case("Monday", "2024-05-13 00:00:00", true, 0)]
    #[case("Monday", "2024-05-13 08:44:59", true, 0)] // boundary
    #[case("Tuesday", "2024-05-14 08:45:00", true, 0)] // boundary
    #[case("Tuesday", "2024-05-14 12:00:00", true, 0)] // representive
    #[case("Tuesday", "2024-05-14 17:59:59", true, 0)] // boundary
    #[case("Wednesday", "2024-05-15 18:00:00", true, 0)] // boundary
    #[case("Saturday", "2024-05-18 12:00:00", true, 0)]
    fn test_solve(#[case] day_of_week: String, #[case] time: String, #[case] membership: bool, #[case] expected: i32) {
        assert_eq!(solve(day_of_week, time, membership), expected);
    }
}