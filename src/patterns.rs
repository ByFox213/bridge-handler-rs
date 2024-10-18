use regex::Regex;
use once_cell::sync::Lazy;
use crate::model::RegexModel;
use crate::util::template;


pub static DD_PATTERNS: Lazy<Vec<RegexModel>> = Lazy::new(|| {
    vec![
        RegexModel::new("trainfngChatRegex", Regex::new(r"\[.*?]\[chat]: \d+:-?\d+:(.*): (.*)").unwrap(), template("")),
        RegexModel::new("trainfngJoinRegex", Regex::new(r"\[.*]\[.*]: \*\*\* '(.*)' (.*)").unwrap(), template("")),
        RegexModel::new("teeworldsChatRegex", Regex::new(r"\[chat]: \d+:-?\d+:(.*): (.*)").unwrap(), template("")),
        RegexModel::new("teeworldsLeaveRegex", Regex::new(r"\[game]: leave player='\d+:(.*)'").unwrap(), template("{{text_leave}}")),
        RegexModel::new("teeworldsJoinRegex", Regex::new(r"\[game]: team_join player='\d+:(.*)' team=0").unwrap(), template("{{text_join}}")),
        RegexModel::new("ddnetChatRegex", Regex::new(r".* I chat: \d+:-?\d+:(.*): (.*)").unwrap(), template("")),
        RegexModel::new("ddnetJoinRegex", Regex::new(r".* I chat: \*\*\* '(.*?)' (.*)").unwrap(), template(""))
    ]
});