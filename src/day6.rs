type Question = char;
type Response = Vec<Question>;
type GroupResponse = Vec<Response>;

fn mk_question(ch: char) -> Option<char> {
    if ('a'..='z').contains(&ch) {
        Some(ch)
    } else {
        None
    }
}

fn count_answers_in_group(group: &GroupResponse) -> usize {
    let mut grp = group
                    .iter()
                    .map(|res| res.iter())
                    .flatten()
                    .collect::<Vec<&Question>>();
    grp.sort();
    grp.dedup();
    grp.len()
}

fn count_unanimous_answers(group: &GroupResponse) -> usize {
    let grp_size = group.len();
    if grp_size == 1 {
        let mut grps = group.concat().clone();
        grps.dedup();
        return grps.len()
    }
    let mut grp = group
                    .iter()
                    .map(|res| res.iter())
                    .flatten()
                    .collect::<Vec<&Question>>();
    grp.sort();
    let (count, _, local_count) = grp.iter().fold((0, ' ', 1), |(count, last, local_count), val| {
        if last == **val {
            (count, **val, local_count + 1)
        } else {
            if local_count == grp_size {
                (count + 1, **val, 1)
            } else {
                (count, **val, 1)
            }
        }
    });
    count + (if local_count == grp_size {1} else {0})
}

fn count_all_answers(all_groups: Vec<GroupResponse>) -> usize {
    all_groups.iter().fold(0, |sum, group| {
        sum + count_answers_in_group(&group)
    })
}

fn count_all_unanimous_answers(all_groups: Vec<GroupResponse>) -> usize {
    all_groups.iter().fold(0, |sum, group| {
        sum + count_unanimous_answers(&group)
    })
}
fn parse_input(input: &str) -> Option<Vec<GroupResponse>> {
    input.split("\n\n")
        .map(|group| {
            let res = group.lines()
                .map(|person| {
                    person.chars()
                        .map(mk_question)
                        .collect()
                }).collect();
            res
        })
        .collect()
}

pub fn run_a(input: &str) {
    let responses = parse_input(input).unwrap();
    let answer_count = count_all_answers(responses);
    println!("{:?}", answer_count);
}

pub fn run_b(input: &str) {
    let responses = parse_input(input).unwrap();
    let unanimous_count = count_all_unanimous_answers(responses);
    println!("{:?}", unanimous_count);
}