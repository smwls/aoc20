#[derive(Debug)]
struct Rule {
    low: u8,
    high: u8,
    letter: char
}

#[derive(Debug)]
struct PasswordRulePair {
    rule: Rule,
    password: String
}

fn parse_rule(rule_str: &str) -> Option<Rule> {
    // or you could just use regex!
    let low = rule_str.chars().take_while(|c| *c != '-')
                    .collect::<String>()
                    .parse::<u8>().ok()?;
    let high = rule_str.chars().skip_while(|c| *c != '-')
                    .skip_while(|c| *c == '-')
                    .take_while(|c| *c != ' ')
                    .collect::<String>()
                    .parse::<u8>().ok()?;
    let letter = rule_str.chars().skip_while(|c| *c != ' ')
                    .skip_while(|c| *c == ' ')
                    .next()?;
    Some(Rule{low: low, high: high, letter: letter})
}

fn check_password(rule_pair: &PasswordRulePair) -> bool {
    let PasswordRulePair{rule, password} = rule_pair;
    let Rule{low, high, letter} = *rule;
    let letter_count = password.chars()
                            .filter(|c| *c == letter)
                            .count() as u8;
    low <= letter_count && letter_count <= high
}

fn check_password_b(rule_pair: &PasswordRulePair) -> bool {
    let PasswordRulePair{rule, password} = rule_pair;
    let Rule{low, high, letter} = *rule;
    let (l, h) = (low as usize, high as usize);
    let letter_str = letter.to_string();
    let first = password.get(l-1..l).unwrap_or("");
    let second = password.get(h-1..h).unwrap_or("");
    if first.len() == 0 || second.len() == 0 {
        false
    } else {
        ((first == letter_str) || (second == letter_str)) && (first != second)
    }
}

fn parse_pair(input: &str) -> Option<PasswordRulePair> {
    let rule_input = input.chars().take_while(|c| *c != ':')
                        .collect::<String>();
    let password = input.chars().skip_while(|c| *c != ':')
                        .skip_while(|c| *c == ':' || *c == ' ')
                        .collect::<String>();
    let parsed_rule = parse_rule(&rule_input)?;
    Some(PasswordRulePair{rule: parsed_rule, password: password})
}

fn parse_input(input: &str) -> Option<Vec<PasswordRulePair>> {
    input.lines().map(|line| {
        parse_pair(line)
    }).collect::<Option<Vec<_>>>()
}

pub fn run_a(input: &str) {
    let parsed_input = parse_input(input).unwrap();
    let valid_pwd_count = parsed_input.iter()
                                .filter(|i| check_password(i))
                                .count();
    println!("{}", valid_pwd_count);
}

pub fn run_b(input: &str) {
    let parsed_input = parse_input(input).unwrap();
    let valid_pwd_count = parsed_input.iter()
                                .filter(|i| check_password_b(i))
                                .count();
    println!("{}", valid_pwd_count);
}