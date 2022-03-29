use std::cmp::max;
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq, Eq, Copy, Clone)]
enum Axis {
    X,
    Y,
}
use Axis::*;
#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

struct Move {
    axis: Axis,
    coord: usize,
}

impl Move {
    fn new(axis: Axis, coord: usize) -> Self {
        Self { axis, coord }
    }
}

struct Game {
    coords: HashSet<Coord>,
    moves: Vec<Move>,
}

impl Game {
    fn new() -> Self {
        Self {
            coords: HashSet::new(),
            moves: Vec::new(),
        }
    }
}
pub fn main() {
    let mut game = parse_data();
    while !game.moves.is_empty() {
        make_move(&mut game);
    }

    let c = find_biggest_x_and_y_coords(&game);
    let (x, y) = (c.x, c.y);
    //    let mut board: Vec<Vec<bool>> = Vec::

    let mut board = vec![vec![false; x + 1]; y + 1];

    for coord in &game.coords {
        board[coord.y][coord.x] = true;
    }

    let board = Board::from_contents(board);
    println!("{board}");
}

struct Board {
    contents: Vec<Vec<bool>>,
}

impl Board {
    fn from_contents(contents: Vec<Vec<bool>>) -> Self {
        Self { contents }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            let mut res = String::new();

            for row in &self.contents {
                for b in row.iter() {
                    if *b {
                        res.push('#');
                    } else {
                        res.push('.');
                    }
                }
                res.push('\n');
            }

            res
        })
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

fn find_biggest_x_and_y_coords(game: &Game) -> Coord {
    let mut biggest_x_coord = 0;
    let mut biggest_y_coord = 0;

    for coord in &game.coords {
        biggest_x_coord = max(coord.x, biggest_x_coord);
        biggest_y_coord = max(coord.y, biggest_y_coord);
    }

    Coord::new(biggest_x_coord, biggest_y_coord)
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct UnmadeChange {
    old: Coord,
    new: Coord,
}

impl UnmadeChange {
    fn new(old: Coord, new: Coord) -> Self {
        Self { old, new }
    }
}
fn make_move(game: &mut Game) {
    let next_move = game.moves.pop();
    let next_move = match next_move {
        None => return,
        Some(v) => v,
    };

    let mut changes_to_be_made = Vec::new();
    for coord in &game.coords {
        let mut coord = *coord;
        let original_coord = coord;
        let applies_for_coord = (next_move.axis == X && coord.x >= next_move.coord)
            || (next_move.axis == Y && coord.y >= next_move.coord);
        if applies_for_coord {
            let relevant_coord = if next_move.axis == X {
                &mut coord.x
            } else {
                &mut coord.y
            };
            let distance_from_divider = *relevant_coord - next_move.coord;
            let new_coord = next_move.coord - distance_from_divider;
            *relevant_coord = new_coord;
            changes_to_be_made.push(UnmadeChange::new(original_coord, coord));
        }
    }
    for unmade_change in changes_to_be_made {
        game.coords.remove(&unmade_change.old);
        game.coords.insert(unmade_change.new);
    }
}
fn parse_data() -> Game {
    let data = std::fs::read_to_string("day13.txt").unwrap();
    let mut lines = data.lines().peekable();
    let mut result = Game::new();
    while match lines.peek() {
        None => None.unwrap(),
        Some(&"") => false,
        _ => true,
    } {
        let next_line = lines.next().unwrap();
        let mut split = next_line.split(",");
        let x_coord = split.next().unwrap().trim().parse::<usize>().unwrap();
        let y_coord = split.next().unwrap().trim().parse::<usize>().unwrap();
        let coord = Coord::new(x_coord, y_coord);
        result.coords.insert(coord);
    }
    for line in lines {
        if line.trim() == "" {
            continue;
        }
        let mut split = line.split("=");

        let first_part = split.next().unwrap();
        let axis = first_part.as_bytes()[first_part.len() - 1];
        let axis = if axis == b'x' { Axis::X } else { Axis::Y };
        let coord = split.next().unwrap().trim().parse::<usize>().unwrap();
        result.moves.push(Move::new(axis, coord));
    }
    result.moves.reverse();
    result
}
