#[derive(Debug, Clone, Copy)]
enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32)
}

#[derive(Debug, Clone, Copy)]
enum ProgramResult {
    Loop(i32),
    Terminated(i32),
    Error
}

type Program = Vec<Op>;

fn exec_program(program: Program) -> ProgramResult {
    let mut seen: Vec<usize> = vec![];
    let mut opcode_index: i32 = 0;
    let mut acc: i32 = 0;
    let result = loop {
        if seen.contains(&&(opcode_index as usize)) {
            break ProgramResult::Loop(acc);
        } else if opcode_index >= program.len() as i32 {
            break ProgramResult::Terminated(acc)
        }
        let x = program.get(opcode_index as usize);
        if let Some(&op) = x {
            seen.push(opcode_index as usize);
            match op {
                Op::Nop(_) => {
                    opcode_index += 1
                },
                Op::Acc(x) => {
                    acc += x;
                    opcode_index += 1;
                },
                Op::Jmp(x) => opcode_index += x
            }
        } else {
            break ProgramResult::Error;
        }
    };
    result
}

fn find_corrupted_op(program: &mut Program) -> Option<i32> {
    let toggle = |op: Op| -> Op {
        match op {
            Op::Nop(x) => Op::Jmp(x),
            Op::Jmp(x) => Op::Nop(x),
            Op::Acc(x) => Op::Acc(x)
        }
    };
    let toggle_program_at = |program: &Program, at: usize| -> Program {
        program.iter().enumerate().map(|(i, val)| {
            if i == at {
                toggle(*val)
            } else {
                *val
            }
        }).collect()
    };
    program.iter()
            .enumerate()
            .filter(|(_, val)| match val {
                Op::Acc(_) => false,
                _ => true
            })
            .map(|(i, _)| i)
            .find_map(|i| {
                match exec_program(toggle_program_at(&program, i)) {
                    ProgramResult::Terminated(x) => Some(x),
                    _ => None
                }
            })
}

fn parse_input(input: &str) -> Option<Program> {
    input.lines().map(|line| {
        let op: String = line.chars()
                    .take_while(|x| *x != ' ')
                    .collect();
        let amount: String = line.chars()
                        .skip_while(|x| *x != ' ')
                        .skip_while(|x| *x == ' ')
                        .collect();
        let parsed_amount: i32 = match amount.chars().next() {
            Some('+') => amount.strip_prefix('+')?.parse().ok(),
            Some('-') => amount.parse().ok(),
            _ => None
        }?;
        match op.as_str() {
            "nop" => Some(Op::Nop(parsed_amount)),
            "acc" => Some(Op::Acc(parsed_amount)),
            "jmp" => Some(Op::Jmp(parsed_amount)),
            _ => None
        }
    }).collect()
}

pub fn run_a(input: &str) {
    let program = parse_input(input).unwrap();
    let output = exec_program(program);
    println!("{:?}", output);
}

pub fn run_b(input: &str) {
    let mut program = parse_input(input).unwrap();
    let output = find_corrupted_op(&mut program);
    println!("{:?}", output);
}