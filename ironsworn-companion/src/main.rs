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

    let handler = dptree::entry()
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

async fn callback_dispatcher(bot: Bot, q: CallbackQuery, app_env: Arc<Env>) -> Result<(), Error> {
    log::info!("debug received query: {:?}", q);

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

                    match &parts[..] {
                        [name, index] => {
                            let index = index.parse::<usize>().unwrap_or_default();
                            let keyboard = make_show_moves_keyboard(app_env, index, name.into());

                            let _ = bot
                                .send_message(chat.id, "Choose the move:")
                                .reply_markup(keyboard)
                                .await;
                        }
                        _ => (),
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

                    log::info!(">>> {:?}", parts);

                    match &parts[..] {
                        [cat_index, _name, index] => {
                            let cat_index = cat_index.parse::<usize>().unwrap_or_default();
                            let index = index.parse::<usize>().unwrap_or_default();
                            if let Some(cat) = app_env.oracles.get_ironsworn_moves().get(cat_index)
                            {
                                if let Some(mov) = cat.moves.get(index) {
                                    let text = format!("{}\n{}", mov.name, mov.text);
                                    let _ = bot.send_message(chat.id, text).await;
                                    // .parse_mode(ParseMode::MarkdownV2)
                                }
                            }
                        }
                        _ => (),
                    };
                } else {
                    log::warn!("CALLBACK WITH DATA NOT HANDLED: {:?}", data);
                }
            }

            log::info!("callback_dispatcher data: {:?}", data);
        }
    }

    Ok(())
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

    match command_handler_factory(cmd_text) {
        Some(iron_handler) => iron_handler.handle(msg, bot, app_env),

        _ => Err(Error::Text(format!(
            "NO COMMAND HANDLER FOUND FOR MESSAGE: {}",
            msg.text().unwrap_or_default()
        ))),
    }
}

fn command_handler_factory(cmd_text: &str) -> Option<&dyn IronHandler> {
    match cmd_text {
        "start" => Some(&(handle_command_start as IronHandlerFn)),

        "help" => Some(&(handle_command_help as IronHandlerFn)),

        "show_moves_categories" => Some(&(handle_show_moves_categories as IronHandlerFn)),

        "roll" => Some(&(handle_command_roll as IronHandlerFn)),
        "roll_100" => Some(&(handle_command_roll_100 as IronHandlerFn)),

        "roll_action" => Some(&(handle_command_roll_action as IronHandlerFn)),
        "roll_theme" => Some(&(handle_command_roll_theme as IronHandlerFn)),
        "roll_action_and_theme" => Some(&(handle_command_roll_action_and_theme as IronHandlerFn)),

        "ask_the_oracle" => Some(&(handle_command_ask_the_oracle as IronHandlerFn)),

        "roll_region" => Some(&(handle_command_roll_region as IronHandlerFn)),
        "roll_location" => Some(&(handle_command_roll_location as IronHandlerFn)),
        "roll_coastal_waters_location" => {
            Some(&(handle_command_roll_coastal_waters_location as IronHandlerFn))
        }
        "roll_location_descriptors" => {
            Some(&(handle_command_roll_location_descriptors as IronHandlerFn))
        }

        "roll_settlement_name" => Some(&(handle_command_roll_settlement_name as IronHandlerFn)),
        "roll_quick_settlement_name" => {
            Some(&(handle_command_roll_quick_settlement_name as IronHandlerFn))
        }
        "roll_settlement_trouble" => {
            Some(&(handle_command_roll_settlement_trouble as IronHandlerFn))
        }

        "roll_character" => Some(&(handle_command_roll_character as IronHandlerFn)),
        "roll_character_role" => Some(&(handle_command_roll_character_role as IronHandlerFn)),
        "roll_character_goal" => Some(&(handle_command_roll_character_goal as IronHandlerFn)),
        "roll_character_descriptor" => {
            Some(&(handle_command_roll_character_descriptor as IronHandlerFn))
        }

        "roll_ironlander_names" => Some(&(handle_command_roll_ironlander_names as IronHandlerFn)),
        "roll_elf_names" => Some(&(handle_command_roll_elf_names as IronHandlerFn)),
        "roll_giant_names" => Some(&(handle_command_roll_giant_names as IronHandlerFn)),
        "roll_varou_names" => Some(&(handle_command_roll_varou_names as IronHandlerFn)),
        "roll_troll_names" => Some(&(handle_command_roll_troll_names as IronHandlerFn)),

        "roll_challenge_rank" => Some(&(handle_command_roll_challenge_rank as IronHandlerFn)),
        "roll_combat_action" => Some(&(handle_command_roll_combat_action as IronHandlerFn)),
        "roll_major_plot_twist" => Some(&(handle_command_roll_major_plot_twist as IronHandlerFn)),
        "roll_mystic_backlash" => Some(&(handle_command_roll_mystic_backlash as IronHandlerFn)),

        "roll_pay_the_price" => Some(&(handle_command_roll_pay_the_price as IronHandlerFn)),

        "test" => Some(&(handle_command_test as IronHandlerFn)),
        _ => None,
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
