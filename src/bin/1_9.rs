// discount day in a store

fn is_discount_day(_year: u32, _month: u32, day: u32) -> bool {
    if (1..6).contains(&day) || (28..31).contains(&day) {
        return true;
    }
    false
}

fn main() {
    let year = 2024;
    let month = 1;
    let day = 1;
    let is_discount_day = is_discount_day(year, month, day);
    println!("Is discount day? {}", is_discount_day);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(2024, 1, 1, true)] // missed
    #[case(2024, 1, 3, true)]
    #[case(2024, 1, 5, true)]
    #[case(2024, 1, 6, false)]
    #[case(2024, 1, 12, false)]
    #[case(2024, 1, 27, false)]
    #[case(2024, 1, 28, true)]
    #[case(2024, 1, 29, true)] // missed
    #[case(2024, 1, 30, true)] // missed
    #[case(2024, 1, 31, false)]
    fn test_is_discount_day(
        #[case] year: u32,
        #[case] month: u32,
        #[case] day: u32,
        #[case] expected: bool,
    ) {
        assert_eq!(is_discount_day(year, month, day), expected);
    }
}
