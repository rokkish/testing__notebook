// happy hour for beer
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct MyArgs {
    // Use coupon
    #[arg(short, long)]
    coupon: bool,

    // In happy_hour time
    #[arg(short, long)]
    ahappy_hour: bool,
}

fn solve(
    coupon: bool,
    happy_hour: bool,
) -> i32 {
    if coupon {
        return 100;
    }
    if happy_hour {
        return 290;
    }
    490
}

fn main() {
    let args = MyArgs::parse();
    println!("{:?}", solve(args.coupon, args.ahappy_hour))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(true, true, 100)]
    #[case(false, true, 290)]
    #[case(false, false, 490)]
    fn test_solve(#[case] coupon: bool, #[case] happy_hour: bool, #[case] expected: i32) {
        assert_eq!(solve(coupon, happy_hour), expected);
    }
}
