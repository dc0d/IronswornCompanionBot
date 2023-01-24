pub mod dice;
pub mod handler_env;
pub mod handlers;
pub mod utl;

use std::sync::Arc;

use teloxide::{
    prelude::*,
    types::{ParseMode::MarkdownV2, Update},
    utils::command::BotCommands,
};

use handler_env::Env;
use handlers::*;
use utl::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!(">>> starting");

    let app_env = Arc::new(Env::load());

    let bot = Bot::from_env();

    bot.set_my_commands(Command::bot_commands()).await.unwrap();

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(command_dispatcher),
        )
        .branch(Update::filter_message().endpoint(message_dispatcher))
        .branch(Update::filter_callback_query().endpoint(callback_dispatcher));

    log::info!(">>> dispatching");
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![app_env])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn command_dispatcher(
    msg: Message,
    bot: Bot,
    cmd: Command,
    app_env: Arc<Env>,
) -> Result<(), Error> {
    log::info!("command_dispatcher {:?} {:?}", cmd, msg.from());

    let r = match command_handler_factory(cmd) {
        Some(iron_handler) => iron_handler.handle(&msg, &bot, app_env),

        _ => Err(Error::Text(format!(
            "NO COMMAND HANDLER FOUND FOR MESSAGE: {}",
            msg.text().unwrap_or_default()
        ))),
    };

    match r {
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
    }
}

fn command_handler_factory(cmd: Command) -> Option<&'static dyn IronHandler> {
    match cmd {
        Command::START => Some(&(handle_command_start as IronHandlerFn)),

        Command::HELP => Some(&(handle_command_help as IronHandlerFn)),

        Command::SHOW_MOVES_CATEGORIES => Some(&(handle_show_moves_categories as IronHandlerFn)),
        Command::MAKE_CHARACTER => Some(&(handle_command_character as IronHandlerFn)),
        Command::MAKE_LOCATION => Some(&(handle_command_make_location as IronHandlerFn)),

        Command::ROLL => Some(&(handle_command_roll as IronHandlerFn)),
        Command::ROLL_100 => Some(&(handle_command_roll_100 as IronHandlerFn)),

        Command::ACTION => Some(&(handle_command_action as IronHandlerFn)),
        Command::THEME => Some(&(handle_command_theme as IronHandlerFn)),
        Command::ACTION_AND_THEME => Some(&(handle_command_action_and_theme as IronHandlerFn)),

        Command::ASK_THE_ORACLE => Some(&(handle_command_ask_the_oracle as IronHandlerFn)),

        Command::REGION => Some(&(handle_command_region as IronHandlerFn)),
        Command::LOCATION => Some(&(handle_command_location as IronHandlerFn)),
        Command::COASTAL_WATERS_LOCATION => {
            Some(&(handle_command_coastal_waters_location as IronHandlerFn))
        }
        Command::LOCATION_DESCRIPTORS => {
            Some(&(handle_command_location_descriptors as IronHandlerFn))
        }

        Command::SETTLEMENT_NAME => Some(&(handle_command_settlement_name as IronHandlerFn)),
        Command::QUICK_SETTLEMENT_NAME => {
            Some(&(handle_command_quick_settlement_name as IronHandlerFn))
        }
        Command::SETTLEMENT_TROUBLE => Some(&(handle_command_settlement_trouble as IronHandlerFn)),

        Command::CHARACTER_ROLE => Some(&(handle_command_character_role as IronHandlerFn)),
        Command::CHARACTER_GOAL => Some(&(handle_command_character_goal as IronHandlerFn)),
        Command::CHARACTER_DESCRIPTOR => {
            Some(&(handle_command_character_descriptor as IronHandlerFn))
        }

        Command::IRONLANDER_NAMES => Some(&(handle_command_ironlander_names as IronHandlerFn)),
        Command::ELF_NAMES => Some(&(handle_command_elf_names as IronHandlerFn)),
        Command::GIANT_NAMES => Some(&(handle_command_giant_names as IronHandlerFn)),
        Command::VAROU_NAMES => Some(&(handle_command_varou_names as IronHandlerFn)),
        Command::TROLL_NAMES => Some(&(handle_command_troll_names as IronHandlerFn)),

        Command::CHALLENGE_RANK => Some(&(handle_command_challenge_rank as IronHandlerFn)),
        Command::COMBAT_ACTION => Some(&(handle_command_combat_action as IronHandlerFn)),
        Command::MAJOR_PLOT_TWIST => Some(&(handle_command_major_plot_twist as IronHandlerFn)),
        Command::MYSTIC_BACKLASH => Some(&(handle_command_mystic_backlash as IronHandlerFn)),

        Command::PAY_THE_PRICE => Some(&(handle_command_pay_the_price as IronHandlerFn)),

        Command::TEST => Some(&(handle_command_test as IronHandlerFn)),
    }
}

async fn callback_dispatcher(bot: Bot, q: CallbackQuery, app_env: Arc<Env>) -> Result<(), Error> {
    // TODO: refactor to callback handlers.
    // what are the shapes of different callback handlers?
    log::info!("debug received query: {:?} {:?}", q.data, q.from);

    match q.data {
        None => (),
        Some(data) => {
            if data.starts_with(CQPX_ORCL_ATO) {
                let odd = data
                    .trim_start_matches(CQPX_ORCL_ATO)
                    .parse::<AskTheOracle>()
                    .unwrap();

                let chance = dice::roll_100();
                let resolve = odd.resolve(chance);

                let _ = bot.answer_callback_query(q.id).await;

                if let Some(Message { id, chat, .. }) = q.message {
                    let _ = bot.delete_message(chat.id, id).await;
                    let _ = bot
                        .send_message(chat.id, format!("{}: {} 🎲 {}", odd, resolve, chance))
                        .await;
                } else {
                    log::warn!("CALLBACK WITH DATA NOT HANDLED: {:?}", data);
                }
            } else if data.starts_with(CQPX_LIST_MOVCATS) {
                let _ = bot.answer_callback_query(q.id).await;

                if let Some(Message { id, chat, .. }) = q.message {
                    let _ = bot.delete_message(chat.id, id).await;

                    let parts: Vec<String> = data
                        .trim_start_matches(CQPX_LIST_MOVCATS)
                        .split("::")
                        .map(|x| x.to_string())
                        .collect();

                    if let [name, index] = &parts[..] {
                        let index = index.parse::<usize>().unwrap_or_default();
                        let keyboard = make_show_moves_keyboard(app_env, index, name.into());
                        let _ = bot
                            .send_message(chat.id, "Choose the move:")
                            .reply_markup(keyboard)
                            .await;
                    };
                } else {
                    log::warn!("CALLBACK WITH DATA NOT HANDLED: {:?}", data);
                }
            } else if data.starts_with(CQPX_LIST_MOVS) {
                let _ = bot.answer_callback_query(q.id).await;

                if let Some(Message { id, chat, .. }) = q.message {
                    let _ = bot.delete_message(chat.id, id).await;

                    let parts: Vec<String> = data
                        .trim_start_matches(CQPX_LIST_MOVS)
                        .split("::")
                        .map(|x| x.to_string())
                        .collect();

                    if let [cat_index, _name, index] = &parts[..] {
                        let cat_index = cat_index.parse::<usize>().unwrap_or_default();
                        let index = index.parse::<usize>().unwrap_or_default();
                        if let Some(cat) = app_env.oracles.get_ironsworn_moves().get(cat_index) {
                            if let Some(mov) = cat.moves.get(index) {
                                let normalized_text = mov.text.clone();
                                let normalized_text = teloxide::utils::markdown::escape_code(
                                    normalized_text.as_str(),
                                );
                                let normalized_text = format!("```\n{}\n```", normalized_text);

                                let text = format!("*{}*\n\n{}", mov.name, normalized_text);
                                let res1 = bot
                                    .send_message(chat.id, text.as_str())
                                    .parse_mode(MarkdownV2)
                                    .disable_web_page_preview(true)
                                    .disable_notification(true)
                                    .await;
                                match res1 {
                                    Ok(_) => (),
                                    Err(err) => log::error!("{:?}", err),
                                };
                            }
                        }
                    };
                } else {
                    log::warn!("CALLBACK WITH DATA NOT HANDLED: {:?}", data);
                }
            }
        }
    }

    Ok(())
}

async fn message_dispatcher(msg: Message, bot: Bot, app_env: Arc<Env>) -> Result<(), Error> {
    log::info!(
        "received message {} {:?}",
        msg.text().unwrap_or_default(),
        msg.from()
    );

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

//

fn is_text_help_command(msg: &Message) -> bool {
    let text = msg.text().unwrap_or_default().trim().to_lowercase();
    matches!(text.as_str(), "help")
}
