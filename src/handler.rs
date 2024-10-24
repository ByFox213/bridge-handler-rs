use std::process::exit;
use log::error;
use regex::Captures;
use crate::emojis::replace_from_emoji;
use crate::model::{Env, Msg, MsgHandler, RegexModel};
use crate::util::{format_regex, format_text, generate_text};


pub async fn chat_handler(msg: MsgHandler, env: &Env, caps: Captures<'_ >, pattern: &RegexModel) -> String {
    let Some((name, text)) = generate_text(caps, pattern, env) else {
        return String::default()
    };

    let text = format_regex(
        format_text(
            replace_from_emoji(text), env.block_text_in_chat.clone()
        ), env.chat_regex.clone()
    );

    let name = format_regex(
        format_text(name, env.block_text_in_nickname.clone()
        ), env.nickname_regex.clone()
    );

    let send_msg = Msg {
        server_name: Some(msg.server_name.clone()),
        name: Some(name),
        message_thread_id: msg.message_thread_id.clone(),
        regex_type: pattern.name.clone(),
        text: Some(text)
    };

    match serde_json::to_string_pretty(&send_msg) {
        Ok(str) => {str}
        Err(err) => {
            error!("Json Serialize Error: {}", err);
            exit(0);
        }
    }
}
