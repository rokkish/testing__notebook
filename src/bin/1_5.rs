// ticket fee system

fn solve(age: i32) -> i32 {
    match age {
        0..=5 => 0,
        6..=12 => 500,
        13..=17 => 1000,
        18..=i32::MAX => 1500,  // over 18
        _ => 0,                 // under 0 is 0
    }
}


fn main() {
    println!("{}", solve(5));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(-1, 0)]
    #[case(2, 0)]
    #[case(5, 0)]
    #[case(6, 500)]
    #[case(8, 500)]
    #[case(12, 500)]
    #[case(13, 1000)]
    #[case(15, 1000)]
    #[case(17, 1000)]
    #[case(18, 1500)]
    #[case(20, 1500)]
    fn test_solve(#[case] age: i32, #[case] expected: i32) {
        assert_eq!(solve(age), expected);
    }
}
