mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
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
    let part_string = match part {
        Part::A => "a",
        Part::B => "b"
    };
    let run_type_string = match run_type {
        RunType::Test => format!("{}_test", part_string),
        RunType::Full => "".to_string()
    };
    let filename = format!("./inputs/day{}{}.txt", day_string, run_type_string);
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
        (3, Part::A) => day3::run_a(input),
        (3, Part::B) => day3::run_b(input), 
        (4, Part::A) => day4::run_a(input),
        (4, Part::B) => day4::run_b(input),
        (5, Part::A) => day5::run_a(input),
        (5, Part::B) => day5::run_b(input),
        (6, Part::A) => day6::run_a(input),
        (6, Part::B) => day6::run_b(input),
        (7, Part::A) => day7::run_a(input),
        (7, Part::B) => day7::run_b(input),
        (8, Part::A) => day8::run_a(input),
        (8, Part::B) => day8::run_b(input),
        (9, Part::A) => day9::run_a(input),
        (9, Part::B) => day9::run_b(input),
        _ => unimplemented!("not a day-part combination that i've started yet")
    }
}
fn main() {
    let RunInfo{day, run_type, part} = parse_command_line_input().unwrap();
    let input = read_input(day, run_type, part).expect("couldn't read input");
    run_aoc(day, part, &input);
}