use std::iter::successors;
use std::ops::Add;

#[derive(Debug, Clone, PartialEq)]
enum GridCell {
    Tree,
    Square
}

type Row = Vec<GridCell>;
type Grid = Vec<Row>;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Coord {
    right: usize,
    down: usize
}

impl Add for Coord {
    type Output =  Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            right: self.right + other.right,
            down: self.down + other.down,
        }    
    }
}

type Slope = Coord;

fn contents_of_cell_at(grid: &Grid, coord: Coord) -> GridCell {
    let num_rows = grid.len();
    let defined_row_width = grid[(coord.down % num_rows)].len();
    let row_position = coord.right % defined_row_width;
    match grid.get(coord.down) {
        Some(row) => match row.get(row_position) {
            Some(cell) => cell.clone(),
            None => GridCell::Square
        }
        None => GridCell::Square
    }
}

fn get_num_trees_along_slope(grid: &Grid, slope: Slope) -> usize {
    let num_traverses = (grid.len() / slope.down) + 1;
    successors(Some(Coord{right: 0, down: 0}), |crd| Some(*crd + slope))
        .map(|crd| contents_of_cell_at(&grid, crd))
        .take(num_traverses)
        .filter(|x| *x == GridCell::Tree)
        .count()
}

fn parse_input(input: &str) -> Option<Grid> {
    input.lines()
        .map(|char_row| {
            char_row.chars().map(|char| {
                match char {
                    '.' => Some(GridCell::Square),
                    '#' => Some(GridCell::Tree),
                    _ => None
                }
            }).collect::<Option<Row>>()
        }).collect::<Option<Grid>>()
}
pub fn run_a(input: &str) {
    let grid = parse_input(input).unwrap();
    let num_trees = get_num_trees_along_slope(&grid, Slope {right: 3, down: 1});
    println!("{}", num_trees);
}

pub fn run_b(input: &str) {
    let grid = parse_input(input).unwrap();
    let slopes = vec![
        Slope {right: 1, down: 1}, 
        Slope {right: 3, down: 1}, 
        Slope {right: 5, down: 1}, 
        Slope {right: 7, down: 1}, 
        Slope {right: 1, down: 2}
    ].into_iter();
    let product = slopes.fold(1, |acc, x| acc*get_num_trees_along_slope(&grid, x));
    println!("{}", product);
}
