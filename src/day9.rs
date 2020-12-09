type Xmas = Vec<i64>;

fn get_first_non_sum(xs: &Xmas, window: usize) -> Option<i64> {
    xs.windows(window)
      .map(|w| w.iter()
                .map(move |x| {
                        w.iter()
                         .filter(|y| *y != x)
                         .map(|z| *z + x)
                         .collect::<Xmas>()
                }).flatten().collect::<Xmas>())
        .zip(xs.iter().skip(window))
        .filter_map(|(sums, x)| {
            if sums.contains(x) {
                None
            } else {
                Some(*x)
            }
        })
      .next()
}

fn get_contiguous(xs: &Xmas, num: i64) -> Option<i64> {
    let mut lower: usize = 0;
    let mut higher: usize = 1;
    let mut sum = xs[0] + xs[1];
    while lower !=  xs.len() - 2 && higher != xs.len() - 1 && lower < higher {
        if sum == num {
            let slice = &xs[lower..=higher];
            return Some(slice.iter().min().unwrap() + slice.iter().max().unwrap())
        } else if sum < num {
            higher += 1;
            sum = sum + xs[higher];
        } else {
            sum = sum - xs[lower];
            lower += 1;
        }
    }
    None
}

fn parse_input(input: &str) -> Option<Vec<i64>> {
    input.lines().map(|n| n.parse::<i64>().ok()).collect()
}

pub fn run_a(input: &str) {
    let parsed_input = parse_input(input).unwrap();
    let first_nonsum = get_first_non_sum(&parsed_input, 25).unwrap();
    println!("{}", first_nonsum);
}

pub fn run_b(input: &str) {
    let parsed_input = parse_input(input).unwrap();
    let first_nonsum = get_first_non_sum(&parsed_input, 25).unwrap();
    let contiguous = get_contiguous(&parsed_input, first_nonsum).unwrap();
    println!("{}", contiguous);   
}