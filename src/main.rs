use teloxide::prelude::*;

mod notes;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting ping bot...");

    let _bot = Bot::from_env().auto_send();

    teloxide::repl(_bot,  |message : Message, bot : AutoSend<Bot>| async move {
	bot.send_dice(message.chat.id).await?;
	respond(())
    }).await;
}
