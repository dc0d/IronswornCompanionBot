use std::fmt;

/*
Odds                The answer is ‘yes’ if you roll:

Almost Certain      11 or greater
Likely              26 or greater
50/50               51 or greater
Unlikely            76 or greater
Small Chance        91 or greater
*/

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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
            76..=100 => format!("{}", AskTheOracle::Unlikely),
            51..=100 => format!("{}", AskTheOracle::FiftyFifty),
            26..=100 => format!("{}", AskTheOracle::Likely),
            11..=100 => format!("{}", AskTheOracle::AlmostCertain),
            _ => format!("{}", AskTheOracle::NoIdea),
        }
    }

    pub fn resolve(&self, chance: i64) -> YesNo {
        match self {
            AskTheOracle::SmallChance => YesNo::from((91..=100).contains(&chance)),
            AskTheOracle::Unlikely => YesNo::from((76..=100).contains(&chance)),
            AskTheOracle::FiftyFifty => YesNo::from((51..=100).contains(&chance)),
            AskTheOracle::Likely => YesNo::from((26..=100).contains(&chance)),
            AskTheOracle::AlmostCertain => YesNo::from((11..=100).contains(&chance)),
            AskTheOracle::NoIdea => YesNo::No,
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

impl std::str::FromStr for AskTheOracle {
    type Err = ();

    fn from_str(s: &str) -> Result<AskTheOracle, ()> {
        match s {
            "Small Chance" => Ok(AskTheOracle::SmallChance),
            "Unlikely" => Ok(AskTheOracle::Unlikely),
            "50/50" => Ok(AskTheOracle::FiftyFifty),
            "Likely" => Ok(AskTheOracle::Likely),
            "Almost Certain" => Ok(AskTheOracle::AlmostCertain),
            "-- honestly? no idea --" => Ok(AskTheOracle::NoIdea),
            _ => Err(()),
        }
    }
}

//

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum YesNo {
    Yes,
    No,
}

impl From<bool> for YesNo {
    fn from(item: bool) -> Self {
        match item {
            true => YesNo::Yes,
            false => YesNo::No,
        }
    }
}

impl fmt::Display for YesNo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            YesNo::Yes => write!(f, "Yes"),
            YesNo::No => write!(f, "No"),
        }
    }
}

//

// CallbackQueryPrefix
pub const CQPX_ORCL_ATO: &str = "ORCL::ATO::";
pub const CQPX_LIST_MOVCATS: &str = "LIST::MOVCATS::";
pub const CQPX_LIST_MOVS: &str = "LIST::MOVS::";

//

pub const BOT_NAME: &str = "IronswornCompanionBot";

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

    #[test]
    fn test_ask_the_oracle_parse() {
        let test_cases = vec![
            ("Small Chance", AskTheOracle::SmallChance),
            ("Unlikely", AskTheOracle::Unlikely),
            ("50/50", AskTheOracle::FiftyFifty),
            ("Likely", AskTheOracle::Likely),
            ("Almost Certain", AskTheOracle::AlmostCertain),
            ("-- honestly? no idea --", AskTheOracle::NoIdea),
        ];

        for (input, expected_output) in test_cases {
            assert_eq!(input.parse::<AskTheOracle>().unwrap(), expected_output);
        }
    }
}
