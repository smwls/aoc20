mod day1;
mod day2;
use std::path::Path;
use std::fs;
use std::env;

type Day = u8;
fn mk_day(day_str: &str) -> Option<Day> {
    let possible_day = day_str.to_string().parse::<u8>().ok()?;
    if possible_day > 25 {
        None
    } else {
        Some(possible_day)
    }
}

enum RunType {
    Test,
    Full
}

#[derive(Clone, Copy)]
enum Part {
    A,
    B
}

struct RunInfo {
    day: Day,
    run_type: RunType,
    part: Part
}

fn read_input(day: Day, run_type: RunType, part: Part) -> Option<String> {
    let day_string = day.to_string();
    let run_type_string = match run_type {
        RunType::Test => "_test",
        RunType::Full => ""
    };
    let part_string = match part {
        Part::A => "a",
        Part::B => "b"
    };
    let filename = format!("./inputs/day{}{}{}.txt", day_string, part_string, run_type_string);
    let path = Path::new(&filename);
    fs::read_to_string(path).ok()
}

fn parse_command_line_input() -> Option<RunInfo> {
    let args: Vec<String> = env::args().collect();
    let parsed_run_type;
    if args.len() > 4 || args.len() < 3 {
        panic!("incorrect number of argument");
    } else if args.len() == 3 {
        parsed_run_type = RunType::Full
    } else if &args[3].clone() == "test" {
        parsed_run_type = RunType::Test
    } else {
        panic!("expected 'test' argument as second argument");
    }
    let day = &args[1].clone();
    let part = &args[2].clone();
    let parsed_part = match &part[..] {
        "a" => Some(Part::A),
        "b" => Some(Part::B),
        _ => None
    }?;
    let parsed_day = mk_day(&day[..])?;
    Some(RunInfo {day:parsed_day, run_type:parsed_run_type, part:parsed_part})
}

fn run_aoc(day: Day, part: Part, input: &str) {
    match (day, part) {
        (1, Part::A) => day1::run_a(input),
        (1, Part::B) => day1::run_b(input),
        (2, Part::A) => day2::run_a(input),
        (2, Part::B) => day2::run_b(input),
        _ => unimplemented!("not a day-part combination that i've started yet")
    }
}
fn main() {
    let RunInfo{day, run_type, part} = parse_command_line_input().unwrap();
    let input = read_input(day, run_type, part).expect("couldn't read input");
    run_aoc(day, part, &input);
}