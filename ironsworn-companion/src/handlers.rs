use std::sync::Arc;

use teloxide::{payloads::SendMessage, prelude::*, requests::JsonRequest};

use crate::dice;
use crate::handler_env::Env;
use crate::utl::*;

pub fn handle_command_start(
    msg: &Message,
    bot: &Bot,
    _app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    Ok(bot.send_message(msg.chat.id, "Let's get on with our journey! 🎲"))
}

pub fn handle_command_help(
    msg: &Message,
    bot: &Bot,
    _app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    Ok(bot.send_message(msg.chat.id, format!("{}", COMMAN_LIST)))
}

pub fn handle_command_roll(
    msg: &Message,
    bot: &Bot,
    _app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    match read_die_number(msg) {
        Some(n) => {
            let chance = dice::roll(n);
            let text = format!("{}", chance);
            Ok(bot.send_message(msg.chat.id, text))
        }
        None => {
            let chance_6 = dice::roll(6);
            let chance_10_1 = dice::roll(10);
            let chance_10_2 = dice::roll(10);

            let mut text_10 = format!("{} - {}", chance_10_1, chance_10_2);
            if chance_6 > chance_10_1 && chance_6 > chance_10_2 {
                text_10 = text_10 + " 💪";
            }
            if chance_10_1 == chance_10_2 {
                text_10 = text_10 + " 🎁";
            }

            let text = format!("{} 🎲 {}", chance_6, text_10);
            Ok(bot.send_message(msg.chat.id, text))
        }
    }
}

fn read_die_number(msg: &Message) -> Option<i64> {
    let (_cmd_txt, args) =
        teloxide::utils::command::parse_command(msg.text().unwrap_or_default(), BOT_NAME)?;
    let first_arg = args.get(0)?;

    match first_arg.to_string().parse().unwrap_or(-1) {
        -1 => None,
        n => Some(n),
    }
}

pub fn handle_command_roll_100(
    msg: &Message,
    bot: &Bot,
    _app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll(100);

    let text = format!("{}", chance);
    Ok(bot.send_message(msg.chat.id, text))
}

pub fn handle_command_roll_action(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(chosen_action) = app_env.oracles.get_action(chance) {
        let action = chosen_action;
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", action, chance)))
    } else {
        Err(Error::Text(format!("ERROR IN CHOOSING ACTION: {}", chance)))
    }
}

pub fn handle_command_roll_theme(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(chosen_theme) = app_env.oracles.get_theme(chance) {
        let theme = chosen_theme;
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", theme, chance)))
    } else {
        Err(Error::Text(format!("ERROR IN CHOOSING ACTION: {}", chance)))
    }
}

pub fn handle_command_roll_action_and_theme(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance_action = dice::roll_100();
    let chance_theme = dice::roll_100();

    let maybe_action = app_env.oracles.get_action(chance_action);
    let maybe_theme = app_env.oracles.get_theme(chance_theme);

    if let (Some(chosen_action), Some(chosen_theme)) = (maybe_action, maybe_theme) {
        let action = chosen_action;
        let theme = chosen_theme;
        Ok(bot.send_message(
            msg.chat.id,
            format!("{} {} 🎲 {} {}", action, theme, chance_action, chance_theme),
        ))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING ACTION/THEME: {}/{}",
            chance_action, chance_theme
        )))
    }
}

pub fn handle_command_roll_ask_the_oracle(
    msg: &Message,
    bot: &Bot,
    _app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    let text = AskTheOracle::chance_to_text(chance);

    Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", text, chance)))
}

pub fn handle_command_roll_region(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(chosen_region) = app_env.oracles.get_region(chance) {
        let region = chosen_region;
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", region, chance)))
    } else {
        Err(Error::Text(format!("ERROR IN CHOOSING region: {}", chance)))
    }
}

pub fn handle_command_roll_location(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(chosen_location) = app_env.oracles.get_location(chance) {
        let location = chosen_location;
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", location, chance)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING location: {}",
            chance
        )))
    }
}

pub fn handle_command_roll_coastal_waters_location(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(chosen_coastal_waters_location) =
        app_env.oracles.get_coastal_waters_location(chance)
    {
        let coastal_waters_location = chosen_coastal_waters_location;
        Ok(bot.send_message(
            msg.chat.id,
            format!("{} 🎲 {}", coastal_waters_location, chance),
        ))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING coastal_waters_location: {}",
            chance
        )))
    }
}

pub fn handle_command_roll_location_descriptors(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(chosen_location_descriptors) = app_env.oracles.get_location_descriptors(chance) {
        let location_descriptors = chosen_location_descriptors;
        Ok(bot.send_message(
            msg.chat.id,
            format!("{} 🎲 {}", location_descriptors, chance),
        ))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING location_descriptors: {}",
            chance
        )))
    }
}

pub fn handle_command_roll_settlement_name(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance1 = dice::roll_100();
    let chance2 = dice::roll_100();
    if let Some(chosen_settlement_name) = app_env.oracles.get_settlement_name(chance1, chance2) {
        let (_desc, prompt, name) = chosen_settlement_name;
        Ok(bot.send_message(
            msg.chat.id,
            format!("{} 🎲 {} {} \n\n{}", name, chance1, chance2, prompt),
        ))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING settlement_name: {}, {}",
            chance1, chance2,
        )))
    }
}

pub fn handle_command_roll_quick_settlement_name(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance1 = dice::roll_100();
    let chance2 = dice::roll_100();
    if let Some(chosen_settlement_name) =
        app_env.oracles.get_quick_settlement_name(chance1, chance2)
    {
        let (prefix, suffix) = chosen_settlement_name;
        Ok(bot.send_message(
            msg.chat.id,
            format!("{} {} 🎲 {} {}", prefix, suffix, chance1, chance2),
        ))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING quick_settlement_name: {}, {}",
            chance1, chance2,
        )))
    }
}

pub fn handle_command_roll_settlement_trouble(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(chosen_settlement_trouble) = app_env.oracles.get_settlement_trouble(chance) {
        Ok(bot.send_message(
            msg.chat.id,
            format!("{} 🎲 {}", chosen_settlement_trouble, chance),
        ))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING settlement_trouble: {}",
            chance,
        )))
    }
}

pub fn handle_command_roll_character(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance1 = dice::roll_100();
    let chance2 = dice::roll_100();
    let chance3 = dice::roll_100();
    if let (Some(role), Some(goal), Some(descriptor)) = (
        app_env.oracles.get_character_role(chance1),
        app_env.oracles.get_character_goal(chance2),
        app_env.oracles.get_character_descriptor(chance3),
    ) {
        Ok(bot.send_message(
            msg.chat.id,
            format!(
                "{} {} - {} 🎲 {} {} - {}",
                descriptor, role, goal, chance3, chance1, chance2
            ),
        ))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING character: {}, {}, {}",
            chance1, chance2, chance3,
        )))
    }
}

pub fn handle_command_roll_character_role(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance1 = dice::roll_100();
    if let Some(role) = app_env.oracles.get_character_role(chance1) {
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", role, chance1)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING character_role: {}",
            chance1,
        )))
    }
}

pub fn handle_command_roll_character_goal(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance2 = dice::roll_100();
    if let Some(goal) = app_env.oracles.get_character_goal(chance2) {
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", goal, chance2)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING character_goal: {}",
            chance2,
        )))
    }
}

pub fn handle_command_roll_character_descriptor(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance3 = dice::roll_100();
    if let Some(descriptor) = app_env.oracles.get_character_descriptor(chance3) {
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", descriptor, chance3)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING character_descriptor: {}",
            chance3,
        )))
    }
}

pub fn handle_command_roll_ironlander_names(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll(200);
    if let Some(descriptor) = app_env.oracles.get_ironlander_names(chance) {
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", descriptor, chance)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING handle_command_roll_ironlander_names: {}",
            chance,
        )))
    }
}

pub fn handle_command_roll_elf_names(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(descriptor) = app_env.oracles.get_elf_names(chance) {
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", descriptor, chance)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING handle_command_roll_elf_names: {}",
            chance,
        )))
    }
}

pub fn handle_command_roll_giant_names(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(descriptor) = app_env.oracles.get_giant_names(chance) {
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", descriptor, chance)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING handle_command_roll_giant_names: {}",
            chance,
        )))
    }
}

pub fn handle_command_roll_varou_names(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(descriptor) = app_env.oracles.get_varou_names(chance) {
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", descriptor, chance)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING handle_command_roll_varou_names: {}",
            chance,
        )))
    }
}

pub fn handle_command_roll_troll_names(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(descriptor) = app_env.oracles.get_troll_names(chance) {
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", descriptor, chance)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING handle_command_roll_troll_names: {}",
            chance,
        )))
    }
}

pub fn handle_command_roll_challenge_rank(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(descriptor) = app_env.oracles.get_challenge_rank(chance) {
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", descriptor, chance)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING handle_command_roll_challenge_rank: {}",
            chance,
        )))
    }
}

pub fn handle_command_roll_combat_action(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(descriptor) = app_env.oracles.get_combat_action(chance) {
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", descriptor, chance)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING handle_command_roll_combat_action: {}",
            chance,
        )))
    }
}

pub fn handle_command_roll_major_plot_twist(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(descriptor) = app_env.oracles.get_major_plot_twist(chance) {
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", descriptor, chance)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING handle_command_roll_major_plot_twist: {}",
            chance,
        )))
    }
}

pub fn handle_command_roll_mystic_backlash(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(descriptor) = app_env.oracles.get_mystic_backlash(chance) {
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", descriptor, chance)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING handle_command_roll_mystic_backlash: {}",
            chance,
        )))
    }
}

pub fn handle_command_test(
    msg: &Message,
    bot: &Bot,
    _app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let github_sha = option_env!("GITHUB_SHA").unwrap_or("unknown GITHUB_SHA");
    let signal = format!("KavehShahbazian {}", github_sha);
    Ok(bot.send_message(msg.chat.id, signal))
}

#[derive(Debug)]
pub enum Error {
    Text(String),
}

pub type IronHandlerFn =
    fn(msg: &Message, bot: &Bot, app_env: Arc<Env>) -> Result<JsonRequest<SendMessage>, Error>;

impl IronHandler for IronHandlerFn {
    fn handle(
        &self,
        msg: &Message,
        bot: &Bot,
        app_env: Arc<Env>,
    ) -> Result<JsonRequest<SendMessage>, Error> {
        self(msg, bot, app_env)
    }
}

pub trait IronHandler {
    fn handle(
        &self,
        msg: &Message,
        bot: &Bot,
        app_env: Arc<Env>,
    ) -> Result<JsonRequest<SendMessage>, Error>;
}
