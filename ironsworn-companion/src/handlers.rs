use std::sync::Arc;

use teloxide::{
    payloads::SendMessage,
    prelude::*,
    requests::JsonRequest,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};

use crate::dice;
use crate::handler_env::Env;
use crate::utl::*;

pub fn handle_command_start(
    msg: &Message,
    bot: &Bot,
    _app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    Ok(bot.send_message(
        msg.chat.id,
        "Let's get on with our journey! 🎲\n\
        \n\
        You can see a list of command by:
        \n\
        -   Typing '/' to bring up the command menu or\n\
        -   Typing the '/help' command or\n\
        -   Pressing the commands menu button.",
    ))
}

pub fn handle_command_help(
    msg: &Message,
    bot: &Bot,
    _app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let lines: Vec<String> = Command::bot_commands()
        .iter()
        .map(|cmd| format!("{} - {}", cmd.command, cmd.description))
        .collect();

    Ok(bot.send_message(msg.chat.id, lines.join("\n")))
}

pub fn handle_show_moves_categories(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let keyboard = make_show_moves_categories_keyboard(app_env);

    Ok(bot
        .send_message(msg.chat.id, "Choose the moves category:")
        .reply_markup(keyboard))
}

fn make_show_moves_categories_keyboard(app_env: Arc<Env>) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let options = app_env.oracles.get_ironsworn_moves_categories_names();

    for row_options in options.chunks(2) {
        let row = row_options
            .iter()
            .map(|(index, cat_name)| {
                InlineKeyboardButton::callback(
                    cat_name.to_string(),
                    format!("{}{}::{}", CQPX_LIST_MOVCATS, cat_name, index),
                )
            })
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_show_moves_keyboard(
    app_env: Arc<Env>,
    cat_index: usize,
    _name: String,
) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    if let Some(cat) = app_env.oracles.get_ironsworn_moves().get(cat_index) {
        let options: Vec<(usize, String)> = cat
            .moves
            .iter()
            .enumerate()
            .map(|(index, elem)| (index, elem.name.clone()))
            .collect();

        for row_options in options.chunks(2) {
            let row = row_options
                .iter()
                .map(|(index, move_name)| {
                    InlineKeyboardButton::callback(
                        move_name.to_string(),
                        format!("{}{}::{}::{}", CQPX_LIST_MOVS, cat_index, move_name, index),
                    )
                })
                .collect();

            keyboard.push(row);
        }
    }

    InlineKeyboardMarkup::new(keyboard)
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
                text_10 += " 💪";
            }
            if chance_10_1 == chance_10_2 {
                text_10 += " 🎁";
            }

            let text = format!("{} 🎲 {}", chance_6, text_10);
            Ok(bot.send_message(msg.chat.id, text))
        }
    }
}

fn read_die_number(msg: &Message) -> Option<i64> {
    let (_cmd_txt, args) =
        teloxide::utils::command::parse_command(msg.text().unwrap_or_default(), BOT_NAME)?;
    let first_arg = args.first()?;

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

pub fn handle_command_action(
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

pub fn handle_command_theme(
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

pub fn handle_command_action_and_theme(
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

pub fn handle_command_ask_the_oracle(
    msg: &Message,
    bot: &Bot,
    _app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let keyboard = make_ask_the_oracle_keyboard();
    Ok(bot
        .send_message(msg.chat.id, "Choose the odds:")
        .reply_markup(keyboard))
}

fn make_ask_the_oracle_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let options = [
        AskTheOracle::SmallChance,
        AskTheOracle::Unlikely,
        AskTheOracle::FiftyFifty,
        AskTheOracle::Likely,
        AskTheOracle::AlmostCertain,
    ];

    for row_options in options.chunks(2) {
        let row = row_options
            .iter()
            .map(|&opt| {
                InlineKeyboardButton::callback(opt.to_string(), format!("{}{}", CQPX_ORCL_ATO, opt))
            })
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

pub fn handle_command_region(
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

pub fn handle_command_location(
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

pub fn handle_command_coastal_waters_location(
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

pub fn handle_command_location_descriptors(
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

pub fn handle_command_settlement_name(
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

pub fn handle_command_quick_settlement_name(
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

pub fn handle_command_settlement_trouble(
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

pub fn handle_command_make_location(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance1 = dice::roll_100();
    let chance2 = dice::roll_100();
    if let (Some(chosen_location_descriptors), Some(chosen_location)) = (
        app_env.oracles.get_location_descriptors(chance1),
        app_env.oracles.get_location(chance2),
    ) {
        let loc_txt = format!("{} {}", chosen_location_descriptors, chosen_location);
        let dice_txt = format!("{} {}", chance1, chance2);
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", loc_txt, dice_txt)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN MAKING LOCATION: {}, {}",
            chance1, chance2
        )))
    }
}

pub fn handle_command_character(
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

pub fn handle_command_character_role(
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

pub fn handle_command_character_goal(
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

pub fn handle_command_character_descriptor(
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

pub fn handle_command_ironlander_names(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll(200);
    if let Some(descriptor) = app_env.oracles.get_ironlander_names(chance) {
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", descriptor, chance)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING handle_command_ironlander_names: {}",
            chance,
        )))
    }
}

pub fn handle_command_elf_names(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(descriptor) = app_env.oracles.get_elf_names(chance) {
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", descriptor, chance)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING handle_command_elf_names: {}",
            chance,
        )))
    }
}

pub fn handle_command_giant_names(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(descriptor) = app_env.oracles.get_giant_names(chance) {
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", descriptor, chance)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING handle_command_giant_names: {}",
            chance,
        )))
    }
}

pub fn handle_command_varou_names(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(descriptor) = app_env.oracles.get_varou_names(chance) {
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", descriptor, chance)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING handle_command_varou_names: {}",
            chance,
        )))
    }
}

pub fn handle_command_troll_names(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(descriptor) = app_env.oracles.get_troll_names(chance) {
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", descriptor, chance)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING handle_command_troll_names: {}",
            chance,
        )))
    }
}

pub fn handle_command_challenge_rank(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(descriptor) = app_env.oracles.get_challenge_rank(chance) {
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", descriptor, chance)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING handle_command_challenge_rank: {}",
            chance,
        )))
    }
}

pub fn handle_command_combat_action(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(descriptor) = app_env.oracles.get_combat_action(chance) {
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", descriptor, chance)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING handle_command_combat_action: {}",
            chance,
        )))
    }
}

pub fn handle_command_major_plot_twist(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(descriptor) = app_env.oracles.get_major_plot_twist(chance) {
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", descriptor, chance)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING handle_command_major_plot_twist: {}",
            chance,
        )))
    }
}

pub fn handle_command_mystic_backlash(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(descriptor) = app_env.oracles.get_mystic_backlash(chance) {
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", descriptor, chance)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING handle_command_mystic_backlash: {}",
            chance,
        )))
    }
}

pub fn handle_command_pay_the_price(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let chance = dice::roll_100();
    if let Some(descriptor) = app_env.oracles.get_pay_the_price(chance) {
        Ok(bot.send_message(msg.chat.id, format!("{} 🎲 {}", descriptor, chance)))
    } else {
        Err(Error::Text(format!(
            "ERROR IN CHOOSING handle_command_mystic_backlash: {}",
            chance,
        )))
    }
}

pub fn handle_command_test(
    msg: &Message,
    bot: &Bot,
    _app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let github_sha =
        std::env::var("APP_GIT_HASH").unwrap_or_else(|_| "unknown APP_GIT_HASH".to_string());
    let signal = format!("@dc0d {}", github_sha);
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

//

#[derive(BotCommands, Clone, Debug)]
#[command(description = "Commands:", rename_rule = "lowercase")]
#[allow(non_camel_case_types)]
pub enum Command {
    #[command(description = "start")]
    START,
    #[command(description = "help")]
    HELP,

    #[command(description = "roll on 'ASK THE ORACLE' Fate Move (probabilities)")]
    ASK_THE_ORACLE,
    #[command(description = "roll action die + challenge dice (or /roll N like /roll 20)")]
    ROLL,
    #[command(description = "show moves")]
    SHOW_MOVES_CATEGORIES,
    #[command(description = "roll on 'PAY THE PRICE' Fate Move")]
    PAY_THE_PRICE,

    #[command(description = "roll on action and theme oracles")]
    ACTION_AND_THEME,
    #[command(description = "roll on character oracles")]
    MAKE_CHARACTER,
    #[command(description = "roll on location & location descriptors oracles")]
    MAKE_LOCATION,

    #[command(description = "roll on major_plot_twist oracle")]
    MAJOR_PLOT_TWIST,

    #[command(description = "roll a d100")]
    ROLL_100,
    #[command(description = "roll on challenge_rank oracle")]
    CHALLENGE_RANK,
    #[command(description = "roll on combat_action oracle")]
    COMBAT_ACTION,
    #[command(description = "roll on action oracle")]
    ACTION,
    #[command(description = "roll on theme oracle")]
    THEME,
    #[command(description = "roll on region oracle")]
    REGION,
    #[command(description = "roll on location oracle")]
    LOCATION,
    #[command(description = "roll on coastal_waters_location oracle")]
    COASTAL_WATERS_LOCATION,
    #[command(description = "roll on location_descriptors oracle")]
    LOCATION_DESCRIPTORS,
    #[command(description = "roll on settlement_name oracle")]
    SETTLEMENT_NAME,
    #[command(description = "roll on quick_settlement_name oracles")]
    QUICK_SETTLEMENT_NAME,
    #[command(description = "roll on settlement_trouble oracle")]
    SETTLEMENT_TROUBLE,
    #[command(description = "roll on character_role oracle")]
    CHARACTER_ROLE,
    #[command(description = "roll on character_goal oracle")]
    CHARACTER_GOAL,
    #[command(description = "roll on character_descriptor oracle")]
    CHARACTER_DESCRIPTOR,
    #[command(description = "roll on ironlander_names oracle")]
    IRONLANDER_NAMES,
    #[command(description = "roll on elf_names oracle")]
    ELF_NAMES,
    #[command(description = "roll on giant_names oracle")]
    GIANT_NAMES,
    #[command(description = "roll on varou_names oracle")]
    VAROU_NAMES,
    #[command(description = "roll on troll_names oracle")]
    TROLL_NAMES,
    #[command(description = "roll on mystic_backlash oracle")]
    MYSTIC_BACKLASH,
    #[command(description = "-")]
    TEST,
}
