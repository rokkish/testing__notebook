// calculate bmi and test it

fn calc(height: f32, weight: f32) -> f32 {
    let bmi: f32 = weight / (height * height / 10000.0);
    bmi
}
fn category(bmi: f32) -> String {
    if bmi < 0.1 {
        "invalid".to_string();
        panic!("invalid bmi")
    } else if (0.1..18.5).contains(&bmi) {
        "underweight".to_string()
    } else if bmi < 25.0 {
        "normal".to_string()
    } else if bmi < 30.0 {
        "preoverweight".to_string()
    } else if bmi < 35.0 {
        "overweight".to_string()
    } else if bmi < 40.0 {
        "overweight2".to_string()
    } else if (40.0..100.0).contains(&bmi) {
        "overweight3".to_string()
    } else {
        "invalid".to_string();
        panic!("invalid bmi")
    }
}
fn main() {
    let height = 170.0;
    let weight = 65.0;
    let bmi = calc(height, weight);
    let category = category(bmi);
    println!("You are {}.", category);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0.1, "underweight")]
    #[case(10.0, "underweight")]
    #[case(18.4, "underweight")]
    #[case(18.5, "normal")]
    #[case(20.5, "normal")]
    #[case(24.9, "normal")]
    #[case(25.0, "preoverweight")]
    #[case(27.0, "preoverweight")]
    #[case(29.9, "preoverweight")]
    #[case(30.0, "overweight")]
    #[case(33.0, "overweight")]
    #[case(34.9, "overweight")]
    #[case(35.0, "overweight2")]
    #[case(37.0, "overweight2")]
    #[case(39.9, "overweight2")]
    #[case(40.0, "overweight3")]
    #[case(70.0, "overweight3")]
    #[case(99.9, "overweight3")]
    // #[should_panic(expected = "invalid bmi")]
    // #[case(-1.0, "invalid")] // `-` string is not allowed, so this test case is not needed
    #[should_panic(expected = "invalid bmi")]
    #[case(0.0, "invalid")]
    #[should_panic(expected = "invalid bmi")]
    #[case(100.0, "invalid")]
    #[should_panic(expected = "invalid bmi")]
    #[case(110.0, "invalid")]
    fn test_calc(#[case] bmi: f32, #[case] expected: &str) {
        assert_eq!(category(bmi), expected.to_string());
    }
}
