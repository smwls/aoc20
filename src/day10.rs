type Joltage = i32;
type JoltageList = Vec<Joltage>;
#[derive(Debug, PartialEq)]

enum JoltageDifference {
    Zero=0,
    One=1,
    Two=2,
    Three=3
}

type JoltageDiffList = Vec<JoltageDifference>;

fn find_joltage_diffs(jolt: JoltageList) -> Option<JoltageDiffList> {
    let device = jolt.iter().max()? + 3;
    let mut jolt_choices = jolt.clone();
    jolt_choices.sort();
    jolt_choices.push(device);
    jolt_choices.iter()
                .scan(0, |state, &x| {
                    let diff = x - *state;
                    *state = x;
                    Some(diff)
                })
                .map(|diff| {
                    println!("{}", diff);
                    match diff {
                    0 => Some(JoltageDifference::Zero),
                    1 => Some(JoltageDifference::One),
                    2 => Some(JoltageDifference::Two),
                    3 => Some(JoltageDifference::Three),
                    _ => None
                }}).collect()
}

fn count_joltage_arrangements(_jolt: JoltageList) -> i64 {
    let device = _jolt.iter().max().unwrap() + 3;
    let mut jolt = _jolt.clone();
    jolt.sort();
    jolt.push(device);
    jolt.insert(0, 0);
    let mut remove: Vec<i64> = jolt.iter().map(|_| 0).collect();
    let mut keep: Vec<i64> = jolt.iter().map(|_| 1).collect();
    remove[1] = if jolt[2] - jolt[0] < 4 {1} else {0};
    for i in 2..(jolt.len()-1) {
        keep[i] = keep[i-1] + remove[i-1];
        remove[i] = if jolt[i+1] - jolt[i-1] < 4 {
            keep[i-2] + if jolt[i+1] - jolt[i-2] < 4 {
                remove[i-1]
            } else {
                0
            }
        } else {
            0
        };
    }
    keep[jolt.len()-1] = keep[jolt.len()-2] + remove[jolt.len()-2];
    remove[jolt.len()-1] = 0;
    keep.pop().unwrap() + remove.pop().unwrap()
}

fn parse_input(input: &str) -> Option<JoltageList> {
    input.lines()
        .map(|x| x.parse::<Joltage>().ok())
        .collect::<Option<JoltageList>>()
}

pub fn run_a(input: &str) {
    let parsed_input = parse_input(input).unwrap();
    let jdiffs = find_joltage_diffs(parsed_input).unwrap();
    let ones = jdiffs.iter().filter(|x| **x == JoltageDifference::One).count();
    let threes = jdiffs.iter().filter(|x| **x == JoltageDifference::Three).count();
    println!("{}", ones*threes);
}

pub fn run_b(input: &str) {
    let parsed_input = parse_input(input).unwrap();
    let counts = count_joltage_arrangements(parsed_input);
    println!("{}", counts);
}