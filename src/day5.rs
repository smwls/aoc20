use std::iter;

#[derive(Debug, PartialEq, Clone, Copy)]
enum BinSp {
    F,
    B,
    L,
    R
}
fn to_val(bin: BinSp) -> i32 {
    match bin {
        BinSp::F | BinSp::L => 0, 
        BinSp::B | BinSp::R => 1
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Seat {
    row: i32,
    col: i32
}

type BoardingPass = Vec<BinSp>;

fn find_consecutives(bp: &mut Vec<BoardingPass>) -> Vec<(i32, i32)> {
    bp.sort_by(|bp1, bp2| {
        let seat1 = get_seat(bp1);
        let seat2 = get_seat(bp2);
        seat_id(seat1).cmp(&seat_id(seat2))
    });
    bp.iter()
        .map(|a| seat_id(get_seat(&a)))
        .zip(bp.iter().skip(1)
                    .map(|a| seat_id(get_seat(&a))))
        .filter(|&(a, b)| a == b - 2)
        .collect()
}

fn seat_id(seat: Seat) -> i32 {
    let Seat {row, col} = seat;
    row*8 + col
}

fn get_seat(bp: &BoardingPass) -> Seat {
    let row = bp.iter()
                .take(7)
                .zip(iter::successors(Some(64), |x| Some(x/2)))
                .fold(0, |acc, (bp_val, two)| {
                    acc + two*to_val(*bp_val)
                });
    let col = bp.iter()
                .skip(7)
                .take(3)
                .zip(iter::successors(Some(4), |x| Some(x/2)))
                .fold(0, |acc, (bp_val, two)| {
                    acc + two*to_val(*bp_val)
                });
    Seat {row: row, col: col}
}

fn validate_boarding_pass(bp: &BoardingPass) -> Option<BoardingPass> {
    match bp.iter().filter(|x| **x == BinSp::F || **x == BinSp::B).count() {
        7 => Some(7),
        _ => None
    }?;
    match bp.iter()
            .skip(7)
            .filter(|x| **x == BinSp::L || **x == BinSp::R).count() {
                3 => Some(3),
                _ => None
    }?;
    Some(bp.clone())
}

fn parse_input(input: &str) -> Option<Vec<BoardingPass>> {
    input.lines().map(|line| {
        line.chars().map(|ch| {
            match ch {
                'F' => Some(BinSp::F),
                'B' => Some(BinSp::B),
                'L' => Some(BinSp::L),
                'R' => Some(BinSp::R),
                _ => None
            }
        }).collect()
    }).map(|sbp: Option<BoardingPass>| {
        match sbp {
            Some(bp) => {
                validate_boarding_pass(&bp)
            },
            None => None
        }
    }).collect()
}

pub fn run_a(input: &str) {
    let boarding_passes = parse_input(input).unwrap();
    let count = boarding_passes.iter()
                                .map(|bp| get_seat(&bp))
                                .map(|seat| {
        Some(seat_id(seat))
    }).max().unwrap();
    println!("{:?}", count);
}

pub fn run_b(input: &str) {
    let mut boarding_passes = parse_input(input).unwrap();
    let ct = find_consecutives(&mut boarding_passes);
    println!("{:?}", ct);
}