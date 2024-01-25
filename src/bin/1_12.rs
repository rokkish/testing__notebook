// input: length, weight
// output: size category

#[derive(Debug, PartialEq)]
enum SizeCategory {
    Small,
    Medium,
    Large,
    None,
}
enum WeightCategory {
    Light,
    Medium,
    Heavy,
    Overweight,
}
enum LengthCategory {
    Short,
    Medium,
    Long,
    Overlong,
}

fn length_category(length: u32) -> LengthCategory {
    match length {
        0..=60 => LengthCategory::Short,
        61..=80 => LengthCategory::Medium,
        81..=100 => LengthCategory::Long,
        _ => LengthCategory::Overlong,
    }
}
fn weight_category(weight: u32) -> WeightCategory {
    match weight {
        0..=2 => WeightCategory::Light,
        3..=5 => WeightCategory::Medium,
        6..=10 => WeightCategory::Heavy,
        _ => WeightCategory::Overweight,
    }
}

fn size_category(length: u32, weight: u32) -> SizeCategory {
    let length_category: LengthCategory = length_category(length);
    let weight_category: WeightCategory = weight_category(weight);
    match (length_category, weight_category) {
        (LengthCategory::Short, WeightCategory::Light) => SizeCategory::Small,
        (LengthCategory::Short, WeightCategory::Medium) => SizeCategory::Medium,
        (LengthCategory::Short, WeightCategory::Heavy) => SizeCategory::Large,
        (LengthCategory::Short, WeightCategory::Overweight) => SizeCategory::None,

        (LengthCategory::Medium, WeightCategory::Light) => SizeCategory::Medium,
        (LengthCategory::Medium, WeightCategory::Medium) => SizeCategory::Medium,
        (LengthCategory::Medium, WeightCategory::Heavy) => SizeCategory::Large,
        (LengthCategory::Medium, WeightCategory::Overweight) => SizeCategory::None,

        (LengthCategory::Long, WeightCategory::Light) => SizeCategory::Large,
        (LengthCategory::Long, WeightCategory::Medium) => SizeCategory::Large,
        (LengthCategory::Long, WeightCategory::Heavy) => SizeCategory::Large,
        (LengthCategory::Long, WeightCategory::Overweight) => SizeCategory::None,

        (LengthCategory::Overlong, WeightCategory::Light) => SizeCategory::None,
        (LengthCategory::Overlong, WeightCategory::Medium) => SizeCategory::None,
        (LengthCategory::Overlong, WeightCategory::Heavy) => SizeCategory::None,
        (LengthCategory::Overlong, WeightCategory::Overweight) => SizeCategory::None,
    }
}
fn main() {
    let length = 1;
    let weight = 1;
    let size_category = size_category(length, weight);
    println!("Size category: {:?}", size_category);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // combination testing
    // 11x11 = 121 test cases
    // 直線を４つの線分に分けると、代表値が４つ、上端と下端が１つずつ、合わせて12個ある（今回は上限値を除外しているので11個）
    // before: -----X-----X-----X-----
    // after:  a-b-cXd-e-fXg-h-iXj-k-l
    #[rstest]
    #[case(0, 0, SizeCategory::Small)] // w: boundary value
    #[case(0, 1, SizeCategory::Small)] // w: representive value
    #[case(0, 2, SizeCategory::Small)] // w: boundary value
    #[case(0, 3, SizeCategory::Medium)] // w: boundary value
    #[case(0, 4, SizeCategory::Medium)] // w: representive value
    #[case(0, 5, SizeCategory::Medium)] // w: boundary value
    #[case(0, 6, SizeCategory::Large)] // w: boundary value
    #[case(0, 8, SizeCategory::Large)] // w: representive value
    #[case(0, 10, SizeCategory::Large)] // w: boundary value
    #[case(0, 11, SizeCategory::None)] // w: boundary value
    #[case(0, 20, SizeCategory::None)] // w: representive value
    #[case(30, 0, SizeCategory::Small)] // w: boundary value
    // ...
    #[case(60, 0, SizeCategory::Small)] // w: boundary value
    // ...
    #[case(61, 0, SizeCategory::Medium)] // w: boundary value
    // ...
    #[case(70, 0, SizeCategory::Medium)] // w: boundary value
    // ...
    #[case(80, 0, SizeCategory::Medium)] // w: boundary value
    // ...
    #[case(81, 0, SizeCategory::Large)] // w: boundary value
    // ...
    #[case(90, 0, SizeCategory::Large)] // w: boundary value
    // ...
    #[case(100, 0, SizeCategory::Large)] // w: boundary value
    // ...
    #[case(101, 0, SizeCategory::None)] // w: boundary value
    // ...
    #[case(200, 0, SizeCategory::None)] // w: boundary value
    fn test_size_category(
        #[case] length: u32,
        #[case] weight: u32,
        #[case] expected: SizeCategory,
    ) {
        assert_eq!(size_category(length, weight), expected);
    }
}
