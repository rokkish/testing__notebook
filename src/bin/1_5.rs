// ticket fee system

fn solve(age: i32) -> Option<i32> {
    match age {
        0..=5 => Some(0),
        6..=12 => Some(500),
        13..=17 => Some(1000),
        18..=120 => Some(1500), // over 18
        121..=i32::MAX => None, // over 120
        _ => Some(0),           // under 0 is 0
    }
}

fn main() {
    println!("{:?}", solve(5));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(-10, Some(0))] // missing case
    #[case(-1, Some(0))]
    #[case(0, Some(0))] // missing case
    #[case(2, Some(0))]
    #[case(5, Some(0))]
    #[case(6, Some(500))]
    #[case(8, Some(500))]
    #[case(12, Some(500))]
    #[case(13, Some(1000))]
    #[case(15, Some(1000))]
    #[case(17, Some(1000))]
    #[case(18, Some(1500))]
    #[case(20, Some(1500))]
    #[case(120, Some(1500))] // missing case
    #[case(121, None)] // missing case
    #[case(140, None)] // missing case
    fn test_solve(#[case] age: i32, #[case] expected: Option<i32>) {
        assert_eq!(solve(age), expected);
    }
}
