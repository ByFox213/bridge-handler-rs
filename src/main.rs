use std::process::exit;
use dotenv::dotenv;
use futures::StreamExt;
use log::{debug, info, error};
use crate::emojis::replace_from_emoji;
use crate::model::{Env, Msg, MsgHandler};
use crate::patterns::DD_PATTERNS;
use crate::util::{format_regex, format_text, generate_text};

mod emojis;
mod model;
mod patterns;
mod util;

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    match dotenv() {
        Ok(_) => {}
        Err(err) => {error!("Failed open file .env: {}", err)}
    };
    let env = Env::get_env()?;

    env_logger::init();

    let nc = match env.connect_nats().await {
        Ok(nc) => {nc}
        Err(err) => {
            eprintln!("Failed connected to nats: {}", err);
            error!("Failed connected to nats: {}", err);
            exit(0)
        }
    };
    let js = async_nats::jetstream::new(nc.clone());

    let mut subscriber = nc.queue_subscribe("tw.handler", "handler".to_string()).await?;

    info!("Handler started");
    while let Some(message) = subscriber.next().await {
        let msg: MsgHandler = match std::str::from_utf8(&message.payload) {
            Ok(json_string) => serde_json::from_str(json_string).unwrap_or_else(|err| {
                error!("Error deserializing JSON: {}", err);
                MsgHandler::default()
            }),
            Err(err) => {
                error!("Error converting bytes to string: {}", err);
                MsgHandler::default()
            }
        };
        if msg.is_default() {
            continue;
        }

        for pattern in DD_PATTERNS.iter() {
            if !pattern.regex.is_match(&msg.text) {
                continue;
            }
            let caps = pattern.regex.captures(&msg.text).unwrap();
            let Some((name, text)) = generate_text(caps, pattern, &env) else {break};

            let text = format_regex(
                format_text(
                    replace_from_emoji(text), env.block_text_in_chat.clone()
                ), env.chat_regex.clone()
            );

            let name = format_regex(
                format_text(name, env.block_text_in_nickname.clone()
                ), env.nickname_regex.clone()
            );

            let send_msg = Msg::new(
                msg.server_name.clone(),
                name,
                msg.message_thread_id.clone(),
                pattern.name.clone(),
                text
            );

            let json = match serde_json::to_string_pretty(&send_msg) {
                Ok(str) => {str}
                Err(err) => {error!("Json Serialize Error: {}", err); break}
            };

            debug!("sended json to tw.messages: {}", json);
            js.publish("tw.messages", json.into())
                .await
                .expect("Error publish message to tw.messages");
            break
        }
    }


    Ok(())
}