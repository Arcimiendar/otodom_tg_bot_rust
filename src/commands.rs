use std::error::Error;
use teloxide::{prelude::*, utils::command::BotCommands};
use crate::lib::lib::register_user;

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "display this text.")]
    Start,
}



pub async fn start_answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Start => {
            println!("{} subscribe", message.chat.id.0);
            register_user(message.chat.id.0 as i32)?;
            bot.send_message(
                message.chat.id, "ща квартиры полетят"
            ).await?
        }

    };

    Ok(())
}