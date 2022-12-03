use std::fmt;

pub const BOT_NAME: &str = "IronswornCompanionBot";

pub enum AskTheOracle {
    SmallChance,
    Unlikely,
    FiftyFifty,
    Likely,
    AlmostCertain,
    NoIdea,
}

impl AskTheOracle {
    pub fn chance_to_text(chance: i64) -> String {
        match chance {
            91..=100 => format!("{}", AskTheOracle::SmallChance),
            76..=90 => format!("{}", AskTheOracle::Unlikely),
            51..=75 => format!("{}", AskTheOracle::FiftyFifty),
            26..=50 => format!("{}", AskTheOracle::Likely),
            11..=25 => format!("{}", AskTheOracle::AlmostCertain),
            _ => format!("{}", AskTheOracle::NoIdea),
        }
    }
}

impl fmt::Display for AskTheOracle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AskTheOracle::SmallChance => write!(f, "Small Chance"),
            AskTheOracle::Unlikely => write!(f, "Unlikely"),
            AskTheOracle::FiftyFifty => write!(f, "50/50"),
            AskTheOracle::Likely => write!(f, "Likely"),
            AskTheOracle::AlmostCertain => write!(f, "Almost Certain"),
            AskTheOracle::NoIdea => write!(f, "-- honestly? no idea --"),
        }
    }
}

pub const COMMAN_LIST: &str = "
/start - start
/help - show help
/roll - roll action die + challenge dice (or /roll N like /roll 20)
/roll_action_and_theme - roll on action_and_theme oracles
/roll_ask_the_oracle - roll on ask_the_oracle (probabilities)
/roll_100 - roll a d100
/roll_character - roll on character oracles
/roll_major_plot_twist - roll on major_plot_twist oracle
/roll_challenge_rank - roll on challenge_rank oracle
/roll_combat_action - roll on combat_action oracle
/roll_action - roll on action oracle
/roll_theme - roll on theme oracle
/roll_region - roll on region oracle
/roll_location - roll on location oracle
/roll_coastal_waters_location - roll on coastal_waters_location oracle
/roll_location_descriptors - roll on location_descriptors oracle
/roll_settlement_name - roll on settlement_name oracle
/roll_quick_settlement_name - roll on quick_settlement_name oracles
/roll_settlement_trouble - roll on settlement_trouble oracle
/roll_character_role - roll on character_role oracle
/roll_character_goal - roll on character_goal oracle
/roll_character_descriptor - roll on character_descriptor oracle
/roll_ironlander_names - roll on ironlander_names oracle
/roll_elf_names - roll on elf_names oracle
/roll_giant_names - roll on giant_names oracle
/roll_varou_names - roll on varou_names oracle
/roll_troll_names - roll on troll_names oracle
/roll_mystic_backlash - roll on mystic_backlash oracle
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ask_the_oracle_display() {
        let test_cases = vec![
            (AskTheOracle::SmallChance, "Small Chance"),
            (AskTheOracle::Unlikely, "Unlikely"),
            (AskTheOracle::FiftyFifty, "50/50"),
            (AskTheOracle::Likely, "Likely"),
            (AskTheOracle::AlmostCertain, "Almost Certain"),
            (AskTheOracle::NoIdea, "-- honestly? no idea --"),
        ];

        for (input, expected_output) in test_cases {
            assert_eq!(format!("{}", input), expected_output);
        }
    }
}
