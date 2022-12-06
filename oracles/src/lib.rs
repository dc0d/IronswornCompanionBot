pub struct Oracles {
    prompts: oracle_loader::ironsworn_oracles_prompts::Root,
    places: oracle_loader::ironsworn_oracles_place::Root,
    settlements: oracle_loader::ironsworn_oracles_settlement::Root,
    characters: oracle_loader::ironsworn_oracles_character::Root,
    names: oracle_loader::ironsworn_oracles_names::Root,
    turning_points: oracle_loader::ironsworn_oracles_turning_point::Root,
    move_oracles: oracle_loader::ironsworn_move_oracles::Root,
}

impl Oracles {
    pub fn load() -> Oracles {
        Oracles {
            prompts: oracle_loader::load_prompts(),
            places: oracle_loader::load_places(),
            settlements: oracle_loader::load_settlements(),
            characters: oracle_loader::load_characters(),
            names: oracle_loader::load_names(),
            turning_points: oracle_loader::load_turning_points(),
            move_oracles: oracle_loader::load_ironsworn_move_oracles(),
        }
    }

    pub fn get_action(&self, chance: i64) -> Option<String> {
        let oracle = self
            .prompts
            .oracles
            .iter()
            .find(|&item| item.name == "Action")?;
        let item = oracle.oracle_table.get((chance - 1) as usize)?;
        Some(item.description.to_string())
    }

    pub fn get_theme(&self, chance: i64) -> Option<String> {
        let oracle = self
            .prompts
            .oracles
            .iter()
            .find(|&item| item.name == "Theme")?;
        let item = oracle.oracle_table.get((chance - 1) as usize)?;
        Some(item.description.to_string())
    }

    pub fn get_region(&self, chance: i64) -> Option<String> {
        let oracle = self
            .places
            .oracles
            .iter()
            .find(|&item| item.name == "Region")?;
        let item = oracle
            .oracle_table
            .iter()
            .filter(|&item| item.chance >= chance)
            .nth(0)?;
        Some(item.description.to_string())
    }

    pub fn get_location(&self, chance: i64) -> Option<String> {
        let oracle = self
            .places
            .oracles
            .iter()
            .find(|&item| item.name == "Location")?;
        let item = oracle
            .oracle_table
            .iter()
            .filter(|&item| item.chance >= chance)
            .nth(0)?;
        Some(item.description.to_string())
    }

    pub fn get_coastal_waters_location(&self, chance: i64) -> Option<String> {
        let oracle = self
            .places
            .oracles
            .iter()
            .find(|&item| item.name == "Coastal Waters Location")?;
        let item = oracle
            .oracle_table
            .iter()
            .filter(|&item| item.chance >= chance)
            .nth(0)?;
        Some(item.description.to_string())
    }

    pub fn get_location_descriptors(&self, chance: i64) -> Option<String> {
        let oracle = self
            .places
            .oracles
            .iter()
            .find(|&item| item.name == "Location Descriptors")?;
        let item = oracle
            .oracle_table
            .iter()
            .filter(|&item| item.chance >= chance)
            .nth(0)?;
        Some(item.description.to_string())
    }

    pub fn get_settlement_name(
        &self,
        chance1: i64,
        chance2: i64,
    ) -> Option<(String, String, String)> {
        let table = self
            .settlements
            .oracles
            .iter()
            .find(|&item| item.name == "Settlement Name")
            .unwrap()
            .oracle_table
            .iter()
            .filter(|&item| item.chance >= chance1)
            .nth(0)?;

        let name = table
            .oracle_table
            .iter()
            .flatten()
            .filter(|&item2| item2.chance >= chance2)
            .nth(0)?;

        let desc = table.description.to_string();
        let prompt = if let Some(prompt_text) = table.prompt.clone() {
            prompt_text.to_string()
        } else {
            "".to_string()
        };

        Some((desc, prompt, name.description.to_string()))
    }

    pub fn get_quick_settlement_name(
        &self,
        chance1: i64,
        chance2: i64,
    ) -> Option<(String, String)> {
        let oracle = self
            .settlements
            .oracles
            .iter()
            .find(|&item| item.name == "Quick Settlement Name")?;

        let prefix_table = oracle
            .oracles
            .iter()
            .flatten()
            .find(|&item| item.name == "Prefix")?;

        let suffix_table = oracle
            .oracles
            .iter()
            .flatten()
            .find(|&item| item.name == "Suffix")?;

        let prefix = prefix_table
            .oracle_table
            .iter()
            .filter(|&item1| item1.chance >= chance1)
            .nth(0)?;

        let suffix = suffix_table
            .oracle_table
            .iter()
            .filter(|&item2| item2.chance >= chance2)
            .nth(0)?;

        Some((
            prefix.description.to_string(),
            suffix.description.to_string(),
        ))
    }

    pub fn get_settlement_trouble(&self, chance: i64) -> Option<String> {
        let oracle = self
            .settlements
            .oracles
            .iter()
            .find(|&item| item.name == "Settlement Trouble")?;
        let item = oracle
            .oracle_table
            .iter()
            .filter(|&item| item.chance >= chance)
            .nth(0)?;
        Some(item.description.to_string())
    }

    pub fn get_character_role(&self, chance: i64) -> Option<String> {
        let oracle = self
            .characters
            .oracles
            .iter()
            .find(|&item| item.name == "Role")?;
        let item = oracle
            .oracle_table
            .iter()
            .filter(|&item| item.chance >= chance)
            .nth(0)?;
        Some(item.description.to_string())
    }

    pub fn get_character_goal(&self, chance: i64) -> Option<String> {
        let oracle = self
            .characters
            .oracles
            .iter()
            .find(|&item| item.name == "Goal")?;
        let item = oracle
            .oracle_table
            .iter()
            .filter(|&item| item.chance >= chance)
            .nth(0)?;
        Some(item.description.to_string())
    }

    pub fn get_character_descriptor(&self, chance: i64) -> Option<String> {
        let oracle = self
            .characters
            .oracles
            .iter()
            .find(|&item| item.name == "Descriptor")?;
        let item = oracle
            .oracle_table
            .iter()
            .filter(|&item| item.chance >= chance)
            .nth(0)?;
        Some(item.description.to_string())
    }

    pub fn get_ironlander_names(&self, chance200: i64) -> Option<String> {
        let oracle = self
            .names
            .oracles
            .iter()
            .find(|&item| item.name == "Ironlander Names")?;
        let item = oracle
            .oracle_table
            .iter()
            .filter(|&item| item.chance >= chance200)
            .nth(0)?;
        Some(item.description.to_string())
    }

    pub fn get_elf_names(&self, chance: i64) -> Option<String> {
        let oracle = self
            .names
            .oracles
            .iter()
            .find(|&item| item.name == "Elf Names")?;
        let item = oracle
            .oracle_table
            .iter()
            .filter(|&item| item.chance >= chance)
            .nth(0)?;
        Some(item.description.to_string())
    }

    pub fn get_giant_names(&self, chance: i64) -> Option<String> {
        let oracle = self
            .names
            .oracles
            .iter()
            .find(|&item| item.name == "Giant Names")?;
        let item = oracle
            .oracle_table
            .iter()
            .filter(|&item| item.chance >= chance)
            .nth(0)?;
        Some(item.description.to_string())
    }

    pub fn get_varou_names(&self, chance: i64) -> Option<String> {
        let oracle = self
            .names
            .oracles
            .iter()
            .find(|&item| item.name == "Varou Names")?;
        let item = oracle
            .oracle_table
            .iter()
            .filter(|&item| item.chance >= chance)
            .nth(0)?;
        Some(item.description.to_string())
    }

    pub fn get_troll_names(&self, chance: i64) -> Option<String> {
        let oracle = self
            .names
            .oracles
            .iter()
            .find(|&item| item.name == "Troll Names")?;
        let item = oracle
            .oracle_table
            .iter()
            .filter(|&item| item.chance >= chance)
            .nth(0)?;
        Some(item.description.to_string())
    }

    pub fn get_challenge_rank(&self, chance: i64) -> Option<String> {
        let oracle = self
            .turning_points
            .oracles
            .iter()
            .find(|&item| item.name == "Challenge Rank")?;
        let item = oracle
            .oracle_table
            .iter()
            .filter(|&item| item.chance >= chance)
            .nth(0)?;
        Some(item.description.to_string())
    }

    pub fn get_combat_action(&self, chance: i64) -> Option<String> {
        let oracle = self
            .turning_points
            .oracles
            .iter()
            .find(|&item| item.name == "Combat Action")?;
        let item = oracle
            .oracle_table
            .iter()
            .filter(|&item| item.chance >= chance)
            .nth(0)?;
        Some(item.description.to_string())
    }

    pub fn get_major_plot_twist(&self, chance: i64) -> Option<String> {
        let oracle = self
            .turning_points
            .oracles
            .iter()
            .find(|&item| item.name == "Major Plot Twist")?;
        let item = oracle
            .oracle_table
            .iter()
            .filter(|&item| item.chance >= chance)
            .nth(0)?;
        Some(item.description.to_string())
    }

    pub fn get_mystic_backlash(&self, chance: i64) -> Option<String> {
        let oracle = self
            .turning_points
            .oracles
            .iter()
            .find(|&item| item.name == "Mystic Backlash")?;
        let item = oracle
            .oracle_table
            .iter()
            .filter(|&item| item.chance >= chance)
            .nth(0)?;
        Some(item.description.to_string())
    }

    pub fn get_pay_the_price(&self, chance: i64) -> Option<String> {
        let oracle = self
            .move_oracles
            .oracles
            .iter()
            .find(|&item| item.name == "Pay the Price")?;
        let item = oracle
            .oracle_table
            .iter()
            .filter(|&item| item.chance >= chance)
            .nth(0)?;
        Some(item.description.to_string())
    }
}
