use std::env;
use teloxide::{dispatching::dialogue::GetChatId, prelude::*, types::{Administrator, ChatMember, ChatMemberKind, ChatMemberStatus, InputFile}, utils::command::BotCommands, RequestError};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    log::info!("Starting command bot...");
    dotenv().ok();
    
    let bot = Bot::new(env::var("TELEGRAM_BOT_KEY").expect("token not found"));
    Command::repl(bot, answer).await;
}



#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Список поддерживаемых команд: ")]
enum Command {
    #[command(description = "start")]
    Start,
    #[command(description = "Доступные команды")]
    Help,
    #[command(description = "Код для помоши в подсчете поссылок с PinDouDou")]
    ProjectCargo,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            start_ok(&bot, &msg).await?
        },
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::ProjectCargo => { project_cargo(&bot, &msg).await? },
    };

    Ok(())
}

async fn start_ok(bot: &Bot, msg: &Message) -> Result<Message, RequestError> {
    let status = status_mem(&msg, &bot).await?;
    if msg.chat.username().unwrap() == "hacker0309" {
        
    }

    match status {
        ChatMemberStatus::Owner => bot.send_message(msg.chat.id ,format!("Приветствую владелец {}", msg.chat.username().unwrap())).await,
        ChatMemberStatus::Administrator =>  bot.send_message(msg.chat.id ,format!("Приветствую администратора {}",msg.chat.username().unwrap())).await,
        ChatMemberStatus::Member =>  bot.send_message(msg.chat.id ,format!("Приветствую пользватель {}", msg.chat.username().unwrap())).await,
        _  => bot.send_message(msg.chat.id, format!("вы по какой то причине находитесь в черном списке")).await
    }
    
}

async fn project_cargo(bot: &Bot, msg: &Message) -> Result<Message, RequestError> {
    bot.send_document(msg.chat.id, InputFile::file("src\\file_project\\calc_pos.txt")).await;
    bot.send_message(msg.chat.id, format!("Если кто то хочет мне что то предложить пишите @hacker0309")).await
}

async fn status_mem(msg: &Message, bot: &Bot) -> Result<ChatMemberStatus, RequestError>{
    let chat_id = msg.chat.id;
    let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;

    return Ok(chat_member.status());
}