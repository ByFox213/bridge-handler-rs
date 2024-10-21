use std::process::exit;
use dotenv::dotenv;
use futures::StreamExt;
use log::{debug, info, error};
use crate::handler::{chat_handler, generate_console, inf_status_regex, status_handler, tw_status_handler};
use crate::model::{Env, MsgHandler};
use crate::patterns::DD_PATTERNS;

mod emojis;
mod model;
mod patterns;
mod util;
mod handler;

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

            let text = msg.text.clone();
            let caps = pattern.regex.captures(&text).unwrap();

            let json = match pattern.name.as_str() {
                "TWStatusRegex" => {tw_status_handler(msg, caps, pattern).await},
                "InfStatusRegex" => {inf_status_regex(msg, caps, pattern).await},
                "SStatusRegex" => {status_handler(msg, caps, pattern, false).await},
                "StatusRegex" => {status_handler(msg, caps, pattern, true).await},
                "console" => {generate_console(msg, caps)}
                _ => {chat_handler(msg, &env, caps, pattern).await}
            };

            if json.is_empty() {
                break
            }


            debug!("sended json to tw.messages: {}", json);
            js.publish("tw.messages", json.into())
                .await
                .expect("Error publish message to tw.messages");
            break
        }
    }


    Ok(())
}