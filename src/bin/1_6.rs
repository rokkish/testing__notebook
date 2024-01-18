// have sield ? in gaming system

enum Weapon {
    ZelcovaSword(String),
    CarrotSword(String),
    WoodSword(String),
    DragonSpear(String),
    KusanagiSword(String),
    None,
}

fn can_equip_weapon(select_weapon: Weapon, have_sield: bool) -> bool {
    if !have_sield {
        return true;
    }
    match select_weapon {
        Weapon::ZelcovaSword(_) => true,
        Weapon::CarrotSword(_) => true,
        Weapon::WoodSword(_) => false,
        Weapon::DragonSpear(_) => false,
        Weapon::KusanagiSword(_) => true,
        Weapon::None => true,
    }
}

fn is_msg_can_not_equip_weapon(select_weapon: Weapon, have_sield: bool) -> bool {
    !can_equip_weapon(select_weapon, have_sield)
}

fn main() {
    println!(
        "{:?}",
        is_msg_can_not_equip_weapon(Weapon::ZelcovaSword(String::from("ZelcovaSword")), true)
    );
    println!(
        "{:?}",
        is_msg_can_not_equip_weapon(Weapon::CarrotSword(String::from("CarrotSword")), true)
    );
    println!(
        "{:?}",
        is_msg_can_not_equip_weapon(Weapon::WoodSword(String::from("WoodSword")), true)
    );
    println!(
        "{:?}",
        is_msg_can_not_equip_weapon(Weapon::DragonSpear(String::from("DragonSpear")), true)
    );
    println!(
        "{:?}",
        is_msg_can_not_equip_weapon(Weapon::KusanagiSword(String::from("KusanagiSword")), true)
    );
    println!("{:?}", is_msg_can_not_equip_weapon(Weapon::None, true));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    // not have sield
    // #[case(Weapon::ZelcovaSword(String::from("ZelcovaSword")), false, false)]
    // #[case(Weapon::CarrotSword(String::from("CarrotSword")), false, false)]
    // #[case(Weapon::WoodSword(String::from("WoodSword")), false, false)]
    // #[case(Weapon::DragonSpear(String::from("DragonSpear")), false, false)]
    // #[case(Weapon::KusanagiSword(String::from("KusanagiSword")), false, false)]
    // #[case(Weapon::None, false, false)]
    // have sield
    #[case(Weapon::ZelcovaSword(String::from("ZelcovaSword")), true, false)]
    // #[case(Weapon::CarrotSword(String::from("CarrotSword")), true, false)]
    // #[case(Weapon::KusanagiSword(String::from("KusanagiSword")), true, false)]
    // #[case(Weapon::None, true, false)]
    // have sield && cant equip weapon
    #[case(Weapon::WoodSword(String::from("WoodSword")), true, true)]
    // #[case(Weapon::DragonSpear(String::from("DragonSpear")), true, true)]
    fn test_equip_sield(
        #[case] select_weapon: Weapon,
        #[case] have_sield: bool,
        #[case] expected: bool,
    ) {
        assert_eq!(
            is_msg_can_not_equip_weapon(select_weapon, have_sield),
            expected
        );
    }
    // 教訓：不要なテストケースを書かない
}
