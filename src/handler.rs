use log::error;
use regex::Captures;
use crate::emojis::replace_from_emoji;
use crate::model::{DataStatus, Env, Msg, MsgHandler, RegexModel};
use crate::util::{format_regex, format_text, generate_text};


pub async fn chat_handler(msg: MsgHandler, env: &Env, caps: Captures<'_ >, pattern: &RegexModel) -> String {
    let Some((name, text)) = generate_text(caps, pattern, &env) else {
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
        data: None,
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
            String::default()
        }
    }
}


pub async fn tw_status_handler(msg: MsgHandler, caps: Captures<'_>, pattern: &RegexModel) -> String {
    let Some(id) = caps.get(1) else { return String::default() };
    let Some(addr) = caps.get(2) else { return String::default() };
    let Some(name) = caps.get(3) else { return String::default() };

    let data = DataStatus {
        time: None,
        user_id: id.as_str().to_string(),
        addr: addr.as_str().to_string(),
        name: name.as_str().to_string(),
        version: None,
    };

    generator(data, msg.server_name.clone(), msg.message_thread_id.clone(), pattern.name.clone())
}

pub async fn inf_status_regex(msg: MsgHandler, caps: Captures<'_>, pattern: &RegexModel) -> String {
    let Some(time) = caps.get(1) else { return String::default() };
    let Some(name) = caps.get(2) else { return String::default() };
    let Some(id) = caps.get(3) else { return String::default() };
    let Some(addr) = caps.get(4) else { return String::default() };
    let Some(version) = caps.get(5) else { return String::default() };

    let data = DataStatus {
        time: Some(time.as_str().to_string()),
        user_id: id.as_str().to_string(),
        addr: addr.as_str().to_string(),
        name: name.as_str().to_string(),
        version: Some(version.as_str().to_string()),
    };

    generator(data, msg.server_name.clone(), msg.message_thread_id.clone(), pattern.name.clone())
}

pub async fn status_handler(msg: MsgHandler, caps: Captures<'_>, pattern: &RegexModel, s: bool) -> String {
    let Some(time) = caps.get(1) else { return String::default() };
    let Some(id) = caps.get(2) else { return String::default() };
    let Some(addr) = caps.get(3) else { return String::default() };
    let Some(name) = caps.get(if s {4} else {5}) else { return String::default() };  // true == name, false == version
    let Some(version) = caps.get(if s {5} else {4}) else { return String::default() };

    let data = DataStatus {
        time: Some(time.as_str().to_string()),
        user_id: id.as_str().to_string(),
        addr: addr.as_str().to_string(),
        name: name.as_str().to_string(),
        version: Some(version.as_str().to_string()),
    };

    generator(data, msg.server_name.clone(), msg.message_thread_id.clone(), pattern.name.clone())
}

fn generator(data: DataStatus, server_name: String, message_thread_id: String, pattern_name: String) -> String {
    let send_msg = Msg {
        data: Some(data),
        server_name: Some(server_name),
        name: None,
        message_thread_id,
        regex_type: pattern_name,
        text: None,
    };

    match serde_json::to_string_pretty(&send_msg) {
        Ok(str) => {str}
        Err(err) => {
            error!("Json Serialize Error: {}", err);
            String::default()
        }
    }
}