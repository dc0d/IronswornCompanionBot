pub mod ironsworn_move_oracles;
pub mod ironsworn_oracles_character;
pub mod ironsworn_oracles_names;
pub mod ironsworn_oracles_place;
pub mod ironsworn_oracles_prompts;
pub mod ironsworn_oracles_settlement;
pub mod ironsworn_oracles_turning_point;

pub fn load_turning_points() -> ironsworn_oracles_turning_point::Root {
    serde_json::from_str(&IRONSWORN_ORACLES_TURNING_POINT).unwrap()
}

const IRONSWORN_ORACLES_TURNING_POINT: &str = include_str!("ironsworn_oracles_turning_point.json");

//

pub fn load_names() -> ironsworn_oracles_names::Root {
    serde_json::from_str(&IRONSWORN_ORACLES_NAMES).unwrap()
}

const IRONSWORN_ORACLES_NAMES: &str = include_str!("ironsworn_oracles_names.json");

//

pub fn load_characters() -> ironsworn_oracles_character::Root {
    serde_json::from_str(&IRONSWORN_ORACLES_CHARACTER).unwrap()
}

const IRONSWORN_ORACLES_CHARACTER: &str = include_str!("ironsworn_oracles_character.json");

#[cfg(test)]
mod tests_load_characters {
    use core::panic;

    use super::*;

    #[test]
    fn smoke_test() {
        let orcl = load_characters();
        if let Some(name) = orcl.oracles.iter().find(|&item| item.name == "Role") {
            assert_eq!(30, name.oracle_table.len());
        } else {
            panic!("Role oracle not found");
        }
    }
}
//

pub fn load_settlements() -> ironsworn_oracles_settlement::Root {
    serde_json::from_str(&IRONSWORN_ORACLES_SETTLEMENT).unwrap()
}

const IRONSWORN_ORACLES_SETTLEMENT: &str = include_str!("ironsworn_oracles_settlement.json");

#[cfg(test)]
mod tests_load_settlements {
    use core::panic;

    use super::*;

    #[test]
    fn smoke_test() {
        let orcl = load_settlements();
        if let Some(name) = orcl
            .oracles
            .iter()
            .find(|&item| item.name == "Settlement Name")
        {
            assert_eq!(7, name.oracle_table.len());
        } else {
            panic!("Settlement Name oracle not found");
        }
    }
}

//

pub fn load_places() -> ironsworn_oracles_place::Root {
    serde_json::from_str(&IRONSWORN_ORACLES_PLACE).unwrap()
}

const IRONSWORN_ORACLES_PLACE: &str = include_str!("ironsworn_oracles_place.json");

#[cfg(test)]
mod tests_load_places {
    use core::panic;

    use super::*;

    #[test]
    fn smoke_test_region_oracle() {
        let orcl = load_places();
        if let Some(regions) = orcl.oracles.iter().find(|&item| item.name == "Region") {
            assert_eq!(10, regions.oracle_table.len());
        } else {
            panic!("region oracle not found");
        }
    }

    #[test]
    fn smoke_test_location_oracle() {
        let orcl = load_places();
        if let Some(locations) = orcl.oracles.iter().find(|&item| item.name == "Location") {
            assert_eq!(51, locations.oracle_table.len());
        } else {
            panic!("location oracle not found");
        }
    }
}

//

pub fn load_prompts() -> ironsworn_oracles_prompts::Root {
    serde_json::from_str(&IRONSWORN_ORACLES_PROMPTS).unwrap()
}

const IRONSWORN_ORACLES_PROMPTS: &str = include_str!("ironsworn_oracles_prompts.json");

#[cfg(test)]
mod tests_load_prompts {
    use core::panic;

    use super::*;

    #[test]
    fn smoke_test_action_oracle() {
        let orcl = load_prompts();
        if let Some(actions) = orcl.oracles.iter().find(|&item| item.name == "Action") {
            assert_eq!(100, actions.oracle_table.len());
        } else {
            panic!("action oracle not found");
        }
    }

    #[test]
    fn smoke_test_theme_oracle() {
        let orcl = load_prompts();
        if let Some(themes) = orcl.oracles.iter().find(|&item| item.name == "Theme") {
            assert_eq!(100, themes.oracle_table.len());
        } else {
            panic!("theme oracle not found");
        }
    }
}

//

pub fn load_ironsworn_move_oracles() -> ironsworn_move_oracles::Root {
    serde_json::from_str(&IRONSWORN_MOVE_ORACLES).unwrap()
}

const IRONSWORN_MOVE_ORACLES: &str = include_str!("ironsworn_move_oracles.json");

#[cfg(test)]
mod tests_ironsworn_move_oracles {
    use core::panic;

    use super::*;

    #[test]
    fn smoke_test_ironsworn_move_oracles() {
        let orcl = load_ironsworn_move_oracles();
        if let Some(actions) = orcl
            .oracles
            .iter()
            .find(|&item| item.name == "Pay the Price")
        {
            assert_eq!(16, actions.oracle_table.len());
        } else {
            panic!("action oracle not found");
        }
    }
}
