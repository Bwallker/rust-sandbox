pub mod lib;
use lib::*;
#[derive(Debug, Copy, Clone)]
struct Point {
    x: u64,
    y: u64,
}
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0 }
    }
}

impl Point {
    fn new(x: u64, y: u64) -> Self {
        Point { x, y }
    }
    fn difference(&self) -> u64 {
        difference(self.x, self.y)
    }
}
#[derive(Debug, Copy, Clone)]
struct Line {
    start: Point,
    end: Point,
}
impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Default for Line {
    fn default() -> Self {
        Line {
            start: Point::default(),
            end: Point::default(),
        }
    }
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        Line { start, end }
    }
    fn from_coords(start_x: u64, start_y: u64, end_x: u64, end_y: u64) -> Self {
        Line {
            start: Point::new(start_x, start_y),
            end: Point::new(end_x, end_y),
        }
    }
}
#[derive(Debug, Clone)]
struct ExpandedLine {
    points: Vec<Point>,
}
impl std::fmt::Display for ExpandedLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Default for ExpandedLine {
    fn default() -> Self {
        ExpandedLine { points: Vec::new() }
    }
}
impl ExpandedLine {
    fn new(points: Vec<Point>) -> Self {
        ExpandedLine { points }
    }
    fn from_line(line: &Line) -> Self {
        Self::new(Self::from_line_helpper(line))
    }
    fn from_line_helpper(line: &Line) -> Vec<Point> {
        if line.start.x == line.end.x {
            if line.start.y == line.end.y {
                return vec![Point::new(line.start.x, line.start.y)];
            } else if line.start.y < line.end.y {
                let mut points = Vec::with_capacity((line.end.y - line.start.y + 1) as usize);
                for i in line.start.y..=line.end.y {
                    points.push(Point::new(line.start.x, i));
                }
                return points;
            } else {
                let mut points = Vec::with_capacity((line.start.y - line.end.y + 1) as usize);
                for i in (line.end.y..=line.start.y).rev() {
                    points.push(Point::new(line.start.x, i));
                }
                return points;
            }
        } else if line.start.y == line.end.y {
            if line.start.x == line.end.x {
                return vec![Point::new(line.start.x, line.start.y)];
            } else if line.start.x < line.end.x {
                let mut points = Vec::with_capacity((line.end.x - line.start.x + 1) as usize);
                for i in line.start.x..=line.end.x {
                    points.push(Point::new(i, line.start.y));
                }
                return points;
            } else {
                let mut points = Vec::with_capacity((line.start.x - line.end.x + 1) as usize);
                for i in (line.end.x..=line.start.x).rev() {
                    points.push(Point::new(i, line.start.y));
                }
                return points;
            }
        } else {
            if line.start.difference() == line.end.difference() {
                if line.start.difference() == 0 {
                    if line.start.x > line.end.x {
                        // 989, 989, 10, 10
                        let mut points =
                            Vec::with_capacity((line.start.x - line.end.x + 1) as usize);
                        for i in (line.end.x..=line.start.x).rev() {
                            points.push(Point::new(i, i));
                        }
                        points
                    } else {
                        // 10, 10, 989, 989
                        let mut points =
                            Vec::with_capacity((line.end.x - line.start.x + 1) as usize);
                        for i in line.start.x..=line.end.x {
                            points.push(Point::new(i, i));
                        }
                        points
                    }
                } else if difference(line.start.x, line.end.y)
                    == difference(line.end.x, line.start.y)
                {
                    if line.start.x > line.end.x {
                        // (971, 36, 36, 971)
                        let mut points =
                            Vec::with_capacity((line.start.x - line.end.x + 1) as usize);
                        for i in (line.end.x..=line.start.x).rev() {
                            points.push(Point::new(i, line.start.y + line.start.x - i));
                        }
                        return points;
                    } else {
                        // (36, 971, 971, 36)
                        let mut points =
                            Vec::with_capacity((line.end.x - line.start.x + 1) as usize);
                        for i in line.start.x..=line.end.x {
                            points.push(Point::new(i, line.start.y + line.start.x - i));
                        }
                        return points;
                    }
                } else {
                    if line.start.x > line.end.x {
                        //686, 465, 263, 42

                        let mut points =
                            Vec::with_capacity((line.start.x - line.end.x + 1) as usize);
                        for i in (line.end.x..=line.start.x).rev() {
                            points.push(Point::new(i, i + line.start.y - line.start.x));
                        }
                        return points;
                    } else {
                        //263, 42, 686, 465

                        let mut points =
                            Vec::with_capacity((line.end.x - line.start.x + 1) as usize);
                        for i in line.start.x..=line.end.x {
                            points.push(Point::new(i, i + line.start.y - line.start.x));
                        }
                        return points;
                    }
                }
            } else {
                if line.start.x < line.end.x {
                    let mut points = Vec::with_capacity((line.end.x - line.start.x + 1) as usize);
                    for i in line.start.x..=line.end.x {
                        // 105, 725, 399, 431
                        points.push(Point::new(i, line.start.y + line.start.x - i));
                    }
                    return points;
                } else {
                    let mut points = Vec::with_capacity((line.start.x - line.end.x + 1) as usize);
                    for i in (line.end.x..=line.start.x).rev() {
                        // 399, 431, 105, 725
                        points.push(Point::new(i, line.start.y + line.start.x - i));
                    }
                    return points;
                }
            }
        }
    }
}

fn difference(num1: u64, num2: u64) -> u64 {
    if num1 > num2 {
        num1 - num2
    } else {
        num2 - num1
    }
}

fn main() {
    let data = get_data();
    let mut grid = generate_grid(1000);
    for line in data {
        let expanded_line = ExpandedLine::from_line(&line);
        for point in expanded_line.points.iter() {
            grid[point.x as usize][point.y as usize] += 1;
        }
    }
    let mut coords: Vec<(usize, usize)> = Vec::new();
    for (i, v) in grid.iter().enumerate() {
        for (j, vv) in v.iter().enumerate() {
            if vv > &1 {
                coords.push((i, j));
            }
        }
    }
    println!("Chunky bois: {:?}", coords.len());
    /*println!(
        "{}",
        ExpandedLine::from_line(&Line::from_coords(3, 4, 6, 7))
    );
    println!(
        "{}",
        ExpandedLine::from_line(&Line::from_coords(6, 7, 3, 4))
    );
    println!(
        "{}",
        ExpandedLine::from_line(&Line::from_coords(981, 951, 192, 162))
    );*/
    /*println!(
        "{}",
        ExpandedLine::from_line(&Line::from_coords(399, 431, 105, 725))
    );
    println!(
        "{}",
        ExpandedLine::from_line(&Line::from_coords(105, 725, 399, 431))
    );
    //273,670 -> 818,125
    println!(
        "{}",
        ExpandedLine::from_line(&Line::from_coords(273, 670, 818, 125))
    );
    println!(
        "{}",
        ExpandedLine::from_line(&Line::from_coords(818, 125, 273, 670))
    );*/
    /*println!(
        "{}",
        ExpandedLine::from_line(&Line::from_coords(263, 42, 686, 465))
    );
    println!(
        "{}",
        ExpandedLine::from_line(&Line::from_coords(686, 465, 263, 42))
    );*/
}

fn generate_grid(size: usize) -> Vec<Vec<u64>> {
    let mut result = Vec::with_capacity(size);
    for _ in 0..size {
        let mut row = Vec::with_capacity(size);
        for _ in 0..size {
            row.push(0);
        }
        result.push(row);
    }
    result
}
fn get_data() -> Vec<Line> {
    let data = read_data();
    let mut result = Vec::new();
    for line in data.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let mut commas = line.split(',');
        let start_x = commas.next().unwrap().trim().parse::<u64>().unwrap();
        let middle_part = commas.next().unwrap().trim();
        let mut arrow = middle_part.split("->");
        let start_y = arrow.next().unwrap().trim().parse::<u64>().unwrap();
        let end_x = arrow.next().unwrap().trim().parse::<u64>().unwrap();
        let end_y = commas.next().unwrap().trim().parse::<u64>().unwrap();
        result.push(Line::from_coords(start_x, start_y, end_x, end_y));
    }
    result
}
