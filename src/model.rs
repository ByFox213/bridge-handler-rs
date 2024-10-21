use std::option::Option;
use std::env;
use std::error::Error;
use async_nats::{Client, ConnectOptions, Error as NatsError};
use liquid::{Template};
use log::debug;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use crate::util::template;


#[derive(Debug, Serialize)]
pub struct DataStatus {
    pub time: Option<String>,
    pub user_id: String,
    pub addr: String,
    pub name: String,
    pub version: Option<String>
}


#[derive(Debug, Serialize)]
pub struct Msg {
    pub data: Option<DataStatus>,
    pub server_name: Option<String>,
    pub name: Option<String>,
    pub message_thread_id: String,
    pub regex_type: String,
    pub text: Option<String>,
}



#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MsgHandler {
    pub server_name: String,
    pub message_thread_id: String,
    pub text: String,
}

impl MsgHandler {
    pub fn is_default(&self) -> bool {
        *self == MsgHandler::default()
    }
}


pub struct Env {
    pub nats_server: String,
    pub nats_user: Option<String>,
    pub nats_password: Option<String>,
    pub text: Template,
    pub text_leave: String,
    pub text_join: String,
    pub nickname_regex: Vec<(Regex, String)>,
    pub block_text_in_nickname: Vec<(String, String)>,
    pub chat_regex: Vec<(Regex, String)>,
    pub block_text_in_chat: Vec<(String, String)>,
}

pub struct RegexModel {
    pub name: String,
    pub regex: Regex,
    pub template: Template,
}

impl RegexModel {
    pub fn new<T: ToString>(name: T, regex: Regex, template: Template) -> RegexModel {
        let name = name.to_string();
        RegexModel {
            name,
            regex,
            template,
        }
    }
}

impl Env {
    pub fn get_env() -> Result<Self, Box<dyn Error + Send + Sync>> {
        debug!("Creating a structure from env");
        Ok(Env {
            nats_server: env::var("nats_server").unwrap_or_else(|_| "nats://127.0.0.1:4222".to_string()),
            nats_user: env::var("nats_user").ok(),
            nats_password: env::var("nats_password").ok(),
            text: template(&env::var("text").unwrap_or_else(|_| "{{player}} {{text}}".to_string())),
            text_leave: env::var("text_leave").unwrap_or_else(|_| "has left the game".to_string()),
            text_join: env::var("text_join").unwrap_or_else(|_| "has join the game".to_string()),
            nickname_regex: env::var("nickname_regex")
                .unwrap_or_else(|_| "".to_string())
                .split(',')
                .filter_map(|s| {
                    let mut parts = s.split(':');
                    let key = parts.next()?.to_string();
                    let value = parts.next()?.to_string();

                    // Use a more robust error handling approach instead of panicking
                    Regex::new(&key).map(|regex| (regex, value)).ok()
                })
                .collect::<Vec<(Regex, String)>>(),
            block_text_in_nickname: env::var("block_text_in_nickname")
                .unwrap_or_else(|_| "tw/:,twitch.tv/:".to_string())
                .split(',')
                .filter_map(|s| {
                    let mut parts = s.split(':');
                    let key = parts.next()?.to_string();
                    let value = parts.next()?.to_string();
                    Some((key, value))
                })
                .collect(),
            chat_regex: env::var("chat_regex")
                .unwrap_or_else(|_| "".to_string())
                .split(',')
                .filter_map(|s| {
                    let mut parts = s.split(':');
                    let key = parts.next()?.to_string();
                    let value = parts.next()?.to_string();

                    // Use a more robust error handling approach instead of panicking
                    Regex::new(&key).map(|regex| (regex, value)).ok()
                })
                .collect::<Vec<(Regex, String)>>(),
            block_text_in_chat: env::var("block_text_in_chat")
                .unwrap_or_else(|_| "".to_string())
                .split(',')
                .filter_map(|s| {
                    let mut parts = s.split(':');
                    let key = parts.next()?.to_string();
                    let value = parts.next()?.to_string();
                    Some((key, value))
                })
                .collect(),
        })
    }

    pub async fn connect_nats(&self) -> Result<Client, NatsError> {
        let nats_user = self.nats_user.clone();
        let nats_password = self.nats_password.clone();

        let connect = match (nats_user, nats_password) {
            (Some(user), Some(password)) => {
                debug!("Connected nats from user and password: {}", self.nats_server);
                ConnectOptions::new().user_and_password(user, password)
            },
            _ => {
                debug!("Connected nats: {}", self.nats_server);
                ConnectOptions::new()
            }
        };

        let nc = connect
            .ping_interval(std::time::Duration::from_secs(15))
            .connect(&self.nats_server)
            .await?;
        Ok(nc)
    }
}