use std::iter;

fn parse_input(input: &str) -> Option<Vec<i32>> {
    input.lines().map(|line| line.parse::<i32>().ok()).collect::<Option<Vec<_>>>()
}

fn find_product_of_pair(input: Vec<i32>) -> Option<i32> {
    let iter = input.iter();
    iter.map(|i| {
        let iter_2 = input.iter().skip_while(move |j| *i + **j != 2020);
        iter_2.zip(iter::repeat(i))
    }).flatten().map(|(i, j)| i * j).next()
}

fn triples<'a>(input: &'a Vec<i32>) -> impl Iterator<Item=(&'a i32, &'a i32, &'a i32)> {
    input.iter()
        .enumerate()
        .map(move |(i, elem_i)| {
            input[i+1..].iter()
                        .enumerate()
                        .map(move |(j, elem_j)| {
                            input[j+1..].iter()
                                        .map(move |elem_k| (elem_i, elem_j, elem_k)) 
                        })
                        .flatten()
        }).flatten()
}

fn find_product_of_triples<'a>(input: &'a Vec<i32>) -> Option<i32> {
    let tuples = triples(input);
    let (a, b, c) = tuples.skip_while(|(a, b, c)| *a + *b + *c != 2020).next()?;
    Some(a*b*c)
}

pub fn run_a(input: &str) {
    let parsed_input = parse_input(input).unwrap();
    match find_product_of_pair(parsed_input) {
        Some(x) => println!("{}", x),
        None => println!("failed")
    }
}

pub fn run_b(input: &str) {
    let parsed_input = parse_input(input).unwrap();
    match find_product_of_triples(&parsed_input) {
        Some(x) => println!("{}", x),
        None => println!("failed")
    }
}