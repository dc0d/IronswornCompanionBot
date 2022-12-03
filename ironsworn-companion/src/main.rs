pub mod dice;
pub mod handler_env;
pub mod handlers;
pub mod utl;

use std::sync::Arc;

use teloxide::{payloads::SendMessage, prelude::*, requests::JsonRequest, types::Update};

use handler_env::Env;
use handlers::*;
use utl::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!(">>> starting");

    let app_env = Arc::new(Env::load());

    let bot = Bot::from_env();

    let handler = dptree::entry().branch(Update::filter_message().endpoint(message_dispatcher));

    log::info!(">>> dispatching");
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![app_env])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn message_dispatcher(msg: Message, bot: Bot, app_env: Arc<Env>) -> Result<(), Error> {
    log::info!("received message {}", msg.text().unwrap_or_default());

    if is_command(&msg) {
        return match command_dispatcher(&msg, &bot, app_env) {
            Ok(result) => {
                let er = result.await;
                match er {
                    Ok(_) => {}
                    Err(err) => {
                        log::error!("AWAITED FUTURE ERRORED: {:?}", err);
                    }
                }
                Ok(())
            }
            Err(err) => Err(err),
        };
    };

    if is_text_help_command(&msg) {
        return match handle_command_help(&msg, &bot, app_env) {
            Ok(result) => {
                let er = result.await;
                match er {
                    Ok(_) => {}
                    Err(err) => {
                        log::error!("AWAITED 'HELP' TEXT FUTURE ERRORED: {:?}", err);
                    }
                }
                Ok(())
            }
            Err(err) => Err(err),
        };
    }

    Err(Error::Text(format!(
        "NO MESSAGE HANDLER FOUND FOR MESSAGE: {}",
        msg.text().unwrap_or_default()
    )))
}

fn command_dispatcher(
    msg: &Message,
    bot: &Bot,
    app_env: Arc<Env>,
) -> Result<JsonRequest<SendMessage>, Error> {
    let (cmd_text, args) =
        teloxide::utils::command::parse_command(msg.text().unwrap_or_default(), BOT_NAME).unwrap();
    log::info!("received command {} {:?}", cmd_text.to_string(), args);

    match cmd_text {
        "start" => handle_command_start(msg, bot, app_env),
        "help" => handle_command_help(msg, bot, app_env),

        "roll" => handle_command_roll(msg, bot, app_env),
        "roll_100" => handle_command_roll_100(msg, bot, app_env),

        "roll_action" => handle_command_roll_action(msg, bot, app_env),
        "roll_theme" => handle_command_roll_theme(msg, bot, app_env),
        "roll_action_and_theme" => handle_command_roll_action_and_theme(msg, bot, app_env),

        "roll_ask_the_oracle" => handle_command_roll_ask_the_oracle(msg, bot, app_env),

        "roll_region" => handle_command_roll_region(msg, bot, app_env),
        "roll_location" => handle_command_roll_location(msg, bot, app_env),
        "roll_coastal_waters_location" => {
            handle_command_roll_coastal_waters_location(msg, bot, app_env)
        }
        "roll_location_descriptors" => handle_command_roll_location_descriptors(msg, bot, app_env),

        "roll_settlement_name" => handle_command_roll_settlement_name(msg, bot, app_env),
        "roll_quick_settlement_name" => {
            handle_command_roll_quick_settlement_name(msg, bot, app_env)
        }
        "roll_settlement_trouble" => handle_command_roll_settlement_trouble(msg, bot, app_env),

        "roll_character" => handle_command_roll_character(msg, bot, app_env),
        "roll_character_role" => handle_command_roll_character_role(msg, bot, app_env),
        "roll_character_goal" => handle_command_roll_character_goal(msg, bot, app_env),
        "roll_character_descriptor" => handle_command_roll_character_descriptor(msg, bot, app_env),

        "roll_ironlander_names" => handle_command_roll_ironlander_names(msg, bot, app_env),
        "roll_elf_names" => handle_command_roll_elf_names(msg, bot, app_env),
        "roll_giant_names" => handle_command_roll_giant_names(msg, bot, app_env),
        "roll_varou_names" => handle_command_roll_varou_names(msg, bot, app_env),
        "roll_troll_names" => handle_command_roll_troll_names(msg, bot, app_env),

        "roll_challenge_rank" => handle_command_roll_challenge_rank(msg, bot, app_env),
        "roll_combat_action" => handle_command_roll_combat_action(msg, bot, app_env),
        "roll_major_plot_twist" => handle_command_roll_major_plot_twist(msg, bot, app_env),
        "roll_mystic_backlash" => handle_command_roll_mystic_backlash(msg, bot, app_env),

        "test" => handle_command_test(msg, bot, app_env),

        _ => Err(Error::Text(format!(
            "NO COMMAND HANDLER FOUND FOR MESSAGE: {}",
            msg.text().unwrap_or_default()
        ))),
    }
}

//

fn is_command(msg: &Message) -> bool {
    match teloxide::utils::command::parse_command(msg.text().unwrap_or_default(), BOT_NAME) {
        None => false,
        Some((_cmd_txt, _args)) => true,
    }
}

fn is_text_help_command(msg: &Message) -> bool {
    let text = msg.text().unwrap_or_default().trim().to_lowercase();
    match text.as_str() {
        "help" => true,
        _ => false,
    }
}
