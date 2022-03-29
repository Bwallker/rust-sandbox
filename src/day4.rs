trait Extract: Default {
    /// Replace self with default and returns the initial value.
    fn extract(&mut self) -> Self;
}

impl<T: Default> Extract for T {
    fn extract(&mut self) -> Self {
        std::mem::replace(self, T::default())
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Cell {
    value: u64,
    marked: bool
}

impl Default for Cell {
    fn default() -> Self {
        Cell{value: 0, marked: false}
    }
}

impl Cell {
    fn new(value: u64) -> Self {
        Cell{value, marked: false}
    }
}
type Row = [Cell; 5];type Board = [Row; 5];
fn print_board(board: &Board) {
    let mut builder: Vec<char> = Vec::new();
    for row in board {
        for cell in row {
            if cell.marked {
                builder.push('[');
            } else {
                builder.push(' ');
            }
            let mut as_str: Vec<char> = cell.value.to_string().chars().collect();
            if as_str.len() == 1 {
                builder.push(' ');
            }
            builder.append(&mut as_str);
            if cell.marked {
                builder.push(']');
            } else {
                builder.push(' ');
            }
            builder.push(' ');
        }
        builder.push('\n');
    }
    let s: String = builder.into_iter().collect();
    println!("{}", s);

}
fn main() {
    let (data_file, numbers_file) = get_file_paths();
    let numbers = get_numbers(numbers_file);
    let mut data = get_data(data_file);
    let mut iter = numbers.iter();
    let mut next;
    let last_winner = loop {
        let winner = loop {
            next = iter.next().unwrap();
            println!("Next number is {}", next);
            match play_round(next, &mut data) {
                Some(v) => break v,
                None => continue,
            };
        };
        println!("Winner found: {} items left", data.len());
        println!("Winner:");
        print_board(&winner);
        data.retain(|x| !check_board(x));
        println!("Items left after culling {}", data.len());
        if data.is_empty() {
            break winner;
        }
    };
    let mut sum = 0;
    for row in last_winner {
        for cell in row {
            if !cell.marked {
                sum += cell.value;
            }
        }
    }
    println!("Last winner is: {:?}", last_winner);
    println!("The sum is: {}", sum);
    println!("The last number was: {}", next);
    println!("The score is: {}", next*sum);

    
}

fn play_round(number: &u64, data: &mut Vec<Board>) -> Option<Board>{
    let mut winner = None;
    for board in data.iter_mut() {
        for row in board.iter_mut() {
            for cell in row.iter_mut() {
                if cell.value == *number {
                    cell.marked = true;
                }
            }
        }
        if check_board(board) {
            if winner == None {
                winner = Some(*board);
            }
        }

    }
    winner
}

fn check_board(board: &Board) -> bool {
    check_rouws(board) || check_columns(board)
}

fn check_rouws(board: &Board) -> bool {
    for row in board.iter() {
        if check_row(row) {
            return true;
        }
    }
    return false;
}


fn check_columns(board: &Board) -> bool {
    for i in 0..5 {
        if check_row(&[board[0][i], board[1][i], board[2][i], board[3][i], board[4][i]]) {
            return true;
        }
    }
    return false;
}

fn check_row(row: &Row) -> bool {
    for cell in row {
        if !cell.marked {
            return false;
        }
    }
    true
}

fn get_file_paths() -> (String, String) {
    let mut args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => ("data.txt".to_string(), "numbers.txt".to_string()),
        3 => (args[1].extract(), args[2].extract()),
        _ => panic!("This script takes either 0 or 2 arguments, the first argument being the path to the data file and the second being the path to the numbers file")
    }
}
fn get_numbers(numbers_file: String) -> Vec<u64> {
    let numbers = match std::fs::read_to_string(&numbers_file) {
        Ok(v) => v,
        Err(_) => panic!("Could not open numbers file using path {{{}}}", numbers_file),
    };
    let mut result = Vec::new();
    for s in numbers.split(",") {
        match s.trim().parse::<u64>() {
            Ok(v) => result.push(v),
            Err(_) => continue,
        };
    };
    result
}


fn get_data(data_file: String) -> Vec<Board> {
    let data = match std::fs::read_to_string(&data_file) {
        Ok(v) => v,
        Err(_) => panic!("Could not open data file using path {{{}}}", data_file),
    };
    let mut result = Vec::new();
    let mut lines = data.lines();
    loop {
        let s = match lines.next() {
            Some(v) => v,
            None => break,
        };
        if s.trim().is_empty() {
            continue;
        };
        let mut r: Board = [[Cell::default(); 5]; 5];
        r[0] = parse_line(s.trim());
        r[1] = parse_line(lines.next().unwrap().trim());
        r[2] = parse_line(lines.next().unwrap().trim());
        r[3] = parse_line(lines.next().unwrap().trim());
        r[4] = parse_line(lines.next().unwrap().trim());
        result.push(r);
    }
    result
}

fn parse_line(line: &str) -> [Cell; 5] {
    let mut r: [Cell; 5] = [Cell::default(); 5];
    let mut nums = line.split_whitespace();
    for i in 0..5 {
        r[i] = construct_cell(&mut nums);
    }
    r
}

fn construct_cell(nums: &mut std::str::SplitWhitespace) -> Cell {
    Cell::new(nums.next().unwrap().parse::<u64>().unwrap())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_1() {
        let numbers = get_numbers("train_numbers.txt".to_string());
        let mut data = get_data("train_data.txt".to_string());
        let mut iter = numbers.iter();
        for _ in 0..11 {
            none_round(&mut iter, &mut data);
        }
        
        some_round(&mut iter, &mut data, 2);

    }

    fn none_round(iter: &mut std::slice::Iter<u64>, data: &mut Vec<Board>) {
        assert_eq!(play_round(iter.next().unwrap(), data), None);
    }

    fn some_round(iter: &mut std::slice::Iter<u64>, data: &mut Vec<Board>, result: usize) {
        let res = play_round(iter.next().unwrap(), data);
        assert_eq!(res, Some(data[result]));
    }
}