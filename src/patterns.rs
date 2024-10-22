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
        RegexModel::new("ddnetJoinRegex", Regex::new(r".* I chat: \*\*\* '(.*?)' (.*)").unwrap(), template("")),
        RegexModel::new("TWStatusRegex", Regex::new(r"\[Server]:\s*id=(\d+)\s*addr=([\d.]+:\d+)\s*name='([^']+)'\s*score=\d+\s*.*\).*").unwrap(), template("")),
        RegexModel::new("TWStatusRegex", Regex::new(r"\[Server]:\s*id=(\d+)\s*addr=([\d.]+:\d+)\s*name='([^']+)'").unwrap(), template("")),
        RegexModel::new("TWStatusRegex", Regex::new(r"\[Server]:\s*id:\s*(\d+)\s+addr:\s*([\d.]+:\d+)\s+secure:\w+\s+score:\d+\s+name:\s*'([^']+)'").unwrap(), template("")),
        RegexModel::new("InfStatusRegex", Regex::new(r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})\s*I\s*Server:\s*\(#\d+\)\s*([^:]+?)\s*:\s*\[antispoof=(\d+)]\s*\[login=\d+]\s*\[level=\d+]\s*\[ip=([\d.]+:\d+)]\s*\[version=(\d+)]\s*\[inf=\d+]").unwrap(), template("")),
        RegexModel::new("SStatusRegex", Regex::new(r"\[(\d{2}:\d{2}:\d{2})]\[server]:\s*id=(\d+)\s*addr=(\d+\.\d+\.\d+\.\d+:\d+)\s*(?:client=(\d+)\s*)?(?:sevendown=(\d+)\s*)?socket=\d+\s*?name='([^']+)'\s*(?:score=\d+\s*)?(?:secure=\w+\s*)?(?:flags=(\d+)\s*)?").unwrap(), template("")),
        RegexModel::new("StatusRegex", Regex::new(r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}|\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}])\s*(?:I\s*server:|\[server]:)\s*id=(\d+\s*) addr=(\d+\.\d+\.\d+\.\d+:\d+\s*) name='([^']+)' \s*(?:client=(\d+\s*))? (?:secure=\w+\s*)? (?:flags=\d+\s*)?").unwrap(), template("")),
        RegexModel::new("StatusRegex", Regex::new(r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}) I server: id=(\d+) addr=<\{([\d.]+:\d+)}> name='([^']+)' client=(\d+)").unwrap(), template("")),
        RegexModel::new("console", Regex::new(r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}) I console: (.+)").unwrap(), template(""))
    ]
});