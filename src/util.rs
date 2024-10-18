use liquid::{ParserBuilder, Template};
use log::debug;
use regex::{Captures, Regex};
use crate::model::{Env, RegexModel};


pub fn template(template: &str) -> Template {
    if !template.is_empty() {
        debug!("template: {}", template);
    }

    ParserBuilder::with_stdlib()
        .build()
        .expect("Template error created")
        .parse(template)
        .expect("Template error parsed")
}


fn format_mention(nickname: String) -> String {
    if nickname.is_empty() {
        return nickname;
    }

    if nickname.contains('@') && nickname.len() > 1 {
        return nickname.replace("@", "@-");
    }
    nickname
}

pub fn generate_text(reg: Captures, pattern: &RegexModel, env: &Env) -> Option<(String, String)> {
    if reg.len() == 3 {
        return Some((
            format_mention(reg.get(1)?.as_str().to_string()),
            reg.get(2)?.as_str().to_string())
        );
    }


    let obj = liquid::object!({
        "text_leave": &env.text_leave,
        "text_join": &env.text_join
    });

    let obj_text = liquid::object!({
        "player": reg.get(1)?.as_str().to_string(),
        "text": pattern.template.render(&obj).expect("Template render error, generate_text")
    });

    let formatted_text = format_mention(
        env.text.render(&obj_text).unwrap()
    );
    Some((String::new(), formatted_text))

}

pub fn format_text(mut text: String, text_vec: Vec<(String, String)>) -> String {
    for (r, t) in text_vec {
        text = text.replacen(&r.to_string(), &t, 1);
    }
    text
}

pub fn format_regex(mut text: String, regex_vec: Vec<(Regex, String)>) -> String {
    for (reg, t) in regex_vec {
        if !reg.is_match(&text) {
            continue;
        }
        let caps = reg.captures(&text).unwrap();
        text = text.replacen(
            &caps.get(1).expect("Format_regex except").as_str().to_string(),
            &t,
            1
        );
    }
    text
}