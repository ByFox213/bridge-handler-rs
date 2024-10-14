use regex::Regex;
use once_cell::sync::Lazy;
use crate::model::RegexModel;
use crate::util::template;


pub static DD_PATTERNS: Lazy<Vec<RegexModel>> = Lazy::new(|| {
    vec![
        RegexModel::new("trainfngChatRegex".to_string(), Regex::new(r"\[.*?]\[chat]: \d+:-?\d+:(.*): (.*)").unwrap(), template("".to_string())),
        RegexModel::new("trainfngJoinRegex".to_string(), Regex::new(r"\[.*]\[.*]: \*\*\* '(.*)' (.*)").unwrap(), template("".to_string())),
        RegexModel::new("teeworldsChatRegex".to_string(), Regex::new(r"\[chat]: \d+:-?\d+:(.*): (.*)").unwrap(), template("".to_string())),
        RegexModel::new("teeworldsLeaveRegex".to_string(), Regex::new(r"\[game]: leave player='\d+:(.*)'").unwrap(), template("{{text_leave}}".to_string())),
        RegexModel::new("teeworldsJoinRegex".to_string(), Regex::new(r"\[game]: team_join player='\d+:(.*)' team=0").unwrap(), template("{{text_join}}".to_string())),
        RegexModel::new("ddnetChatRegex".to_string(), Regex::new(r".* I chat: \d+:-?\d+:(.*): (.*)").unwrap(), template("".to_string())),
        RegexModel::new("ddnetJoinRegex".to_string(), Regex::new(r".* I chat: \*\*\* '(.*?)' (.*)").unwrap(), template("".to_string()))
    ]
});