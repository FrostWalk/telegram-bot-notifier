use std::env;

use teloxide::prelude::*;
use teloxide::types::ChatId;

const TOKEN_VAR: &str = "TOKEN";
const CHAT_ID_VAR: &str = "CHAT_ID";


#[tokio::main]
async fn main() {
    let bot = Bot::new(get_env_var(TOKEN_VAR));

    let chat_id = match get_env_var(CHAT_ID_VAR).parse::<i64>() {
        Ok(i) => i,
        Err(e) => panic!("invalid chat id, required i64 provided {}\n{}", get_env_var(CHAT_ID_VAR), e)
    };

    let chat_id = ChatId(chat_id);

    if env::args().len() <= 1 {
        panic!("no text provided")
    }

    let message_text = env::args().skip(1).collect::<Vec<String>>().join(" ");

    match bot.send_message(chat_id, message_text).send().await {
        Ok(_) => {}
        Err(e) => panic!("unable to send message: {}", e),
    }
}

#[inline(always)]
fn get_env_var(name: &str) -> String {
    match env::var(name) {
        Ok(v) => v,
        Err(e) => panic!("required env variable {} is not set\n({})", name, e)
    }
}