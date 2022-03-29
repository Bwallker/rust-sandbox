use std::cmp::min;
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
const DATA: &'static str = include_str!("../day15.txt");
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Debug for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub fn main() {
    let data = parse_data();
    let best = find_best_path(&data);
    println!("{}", best);
}

fn find_best_path_recurse(
    data: &Vec<Vec<u8>>,
    current_coord: Coordinate,
    current_score: u32,
    mut best_score: u32,
    previous_coords: &mut HashSet<Coordinate>,
) -> u32 {
    if data
        .get(current_coord.x)
        .map(|row| row.get(current_coord.y))
        .flatten()
        .is_none()
    {
        return u32::MAX;
    }
    if previous_coords.contains(&current_coord) {
        return u32::MAX;
    }
    previous_coords.insert(current_coord);
    if current_score > best_score {
        return u32::MAX;
    }

    if current_coord.x == data[0].len() - 1 && current_coord.y == data.len() - 1 {
        best_score = min(best_score, current_score);
    }
    let x = current_coord.x;
    let y = current_coord.y;
    let up = find_best_path_recurse(
        data,
        Coordinate::new(x, y.wrapping_sub(1)),
        current_score + data[x][y] as u32,
        best_score,
        previous_coords,
    );
    let right = find_best_path_recurse(
        data,
        Coordinate::new(x + 1, y),
        current_score + data[x][y] as u32,
        best_score,
        previous_coords,
    );

    let left = find_best_path_recurse(
        data,
        Coordinate::new(x.wrapping_sub(1), y),
        current_score + data[x][y] as u32,
        best_score,
        previous_coords,
    );

    let down = find_best_path_recurse(
        data,
        Coordinate::new(x, y + 1),
        current_score + data[x][y] as u32,
        best_score,
        previous_coords,
    );

    let mut best = min(up, down);
    best = min(best, left);
    best = min(best, right);
    best
}

fn find_best_path(data: &Vec<Vec<u8>>) -> u32 {
    find_best_path_recurse(
        data,
        Coordinate::new(0, 0),
        0,
        u32::MAX,
        &mut HashSet::new(),
    )
}

fn parse_data() -> Vec<Vec<u8>> {
    DATA.lines()
        .map(|x| x.trim().bytes().map(|x| x - b'0').collect())
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn my_test_case() {}
}
