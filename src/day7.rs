use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter;

type BagColour = String;
type BagGraph = HashMap<BagColour, EdgeList>;

type ColourList = Vec<BagColour>;

#[derive(Clone, Debug, PartialEq)]
struct EdgeList {
    ins: ColourList,
    outs: ColourList
}

fn parse_rule(input: &str) -> Option<(BagColour, ColourList)> {
    let node = input.split(" bags contain ").next()?.to_string();
    let edge_str: String = input.split(" bags contain ")
                    .skip(1)
                    .collect();
    if edge_str.contains("no other bags") {
        return Some((node, vec![]))
    }
    let pair_list = edge_str.replace(" bag, ", "|")
                    .replace(" bags, ", "|")
                    .replace(" bag.", "")
                    .replace(" bags.", "")
                    .split('|')
                    .map(|rule| {
        let number = rule.split(' ')
                        .next()?
                        .parse::<usize>().ok()?;
        let colour = rule.strip_prefix(&number.to_string())?.trim_start().to_string();
        Some((number, colour))
    }).collect::<Option<Vec<(usize, BagColour)>>>()?;
    let list = pair_list.iter().map(|(n, c)| -> ColourList {
        iter::repeat_with(|| c.to_string()).take(*n).collect()
    }).collect::<Vec<ColourList>>().concat();
    Some((node, list))
}

fn generate_graph(rule_pairs: Vec<(BagColour, ColourList)>) -> BagGraph {
    let mut graph = BagGraph::new();
    let rules = rule_pairs.iter().map(|(a, b)| (a.to_string(), b.clone()))
                        .collect::<Vec<(BagColour, ColourList)>>();
    rules.iter().for_each(|(colour, list)| {
        let new_list = EdgeList{ins: vec![], outs: list.clone()};
        graph.insert(colour.to_string(),new_list);
    });
    rules.iter().for_each(|(_, list)| {
            let new_list = EdgeList{ins: vec![], outs: vec![]};
            list.iter().for_each(|l| {
                if let None = graph.get(l) {
                    graph.insert(l.to_string(), new_list.clone());
                }
            });
    });
    rules.iter().for_each(|(colour, list)| {
        list.iter().for_each(|l| {
            if let Some(EdgeList{ins, outs: _}) = graph.get_mut(l) {
                ins.push(colour.to_string());
            }
        })
    });
    graph
}

fn get_predecessors(graph: &mut BagGraph, colour: BagColour) -> ColourList {
    let mut seen: Vec<&str> = vec![];
    let mut to_visit: Vec<&str> = vec![&colour];
    while let Some(x) = to_visit.pop() {
        if !seen.contains(&x) {
            match graph.get(x) {
                Some(EdgeList{ins, outs: _}) => {
                    seen.push(x);
                    let mut ins_mut = ins.iter()
                                        .map(|i| i.as_str())
                                        .filter(|i| !seen.contains(&i))
                                        .collect();
                    to_visit.append(&mut ins_mut);
                }
                None => ()
            };
        }
    }
    seen.iter().map(|i| i.to_string()).collect()
}

fn get_successor_count(graph: &mut BagGraph, colour: BagColour) -> usize {
    let mut count: HashMap<BagColour, usize> = HashMap::new();
    let mut to_visit: VecDeque<&str> = graph.iter().filter(|(_, EdgeList{ins: _, outs})| {
        outs.len() == 0
    }).map(|(k, _)| k.as_str()).collect();
    graph.keys().for_each(|key| {
        count.insert(key.to_string(), if to_visit.contains(&key.as_str()) {1} else {0});
    });
    while let Some(x) = to_visit.pop_front() {
        let x_count = *count.get(x).unwrap_or(&0);
        match graph.get(x) {
            Some(EdgeList{ins, outs: _}) => {
                let mut ins_added = vec![];
                for in_ in ins.iter() {
                    match graph.get(in_) {
                        Some(EdgeList{ins: _, outs: in_outs}) => {
                            if !ins_added.contains(in_) {
                                let counts_except_x: usize = in_outs.iter()
                                                        .filter(|y| *y != x)
                                                        .map(|y| count.get(y).unwrap_or(&0))
                                                        .sum();
                                let in_count = ins.iter().filter(|y| *y == in_).count();
                                count.insert(in_.to_string(), 1 + counts_except_x + in_count*x_count);
                                ins_added.push(in_.to_string());
                                to_visit.push_back(in_);
                            }
                        },
                        None => ()
                    }
                }
            },
            None => ()
        }
    }
    *count.get(&colour).unwrap_or(&0) - 1
}   

fn parse_input(input: &str) -> Option<BagGraph> {
    let rules = input.lines()
        .map(parse_rule)
        .collect::<Option<Vec<(BagColour, ColourList)>>>()?;
    Some(generate_graph(rules))
}

pub fn run_a(input: &str) {
    let mut graph = parse_input(input).unwrap();
    let preds = get_predecessors(&mut graph, "shiny gold".to_string());
    println!("{:?}", preds);
}

pub fn run_b(input: &str) {
    let mut graph = parse_input(input).unwrap();
    let succ_count = get_successor_count(&mut graph, "shiny gold".to_string());
    println!("{:?}", succ_count);
}