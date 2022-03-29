use std::collections::HashMap;

const DATA: &'static str = include_str!("../day14.txt");
type Rules = HashMap<Pair, u8>;
type Frequencies = HashMap<Pair, i128>;
type CharacterCount = HashMap<u8, u128>;

struct Game {
    frequencies: Frequencies,
    rules: Rules,
    character_count: CharacterCount,
}

impl Game {
    fn new(frequencies: Frequencies, rules: Rules, character_count: CharacterCount) -> Self {
        Self {
            frequencies,
            rules,
            character_count,
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Pair {
    first: u8,
    second: u8,
}

impl Pair {
    fn new(string: &str) -> Self {
        let string = string.trim();
        let mut bytes = string.bytes();
        let first = bytes.next().unwrap();
        let second = bytes.next().unwrap();
        Self { first, second }
    }

    fn from_bytes(first: u8, second: u8) -> Self {
        Self { first, second }
    }
}
use std::fmt::{Debug, Display, Formatter};
impl Display for Pair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.first as char, self.second as char)
    }
}

impl Debug for Pair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub fn main() {
    let mut game = parse_data();
    let number_of_rounds_to_play: usize = 40;
    for _ in 0..number_of_rounds_to_play {
        play_round(&mut game);
    }
    let (most, least) = find_common_elements(&game.character_count);
    let as_chars: HashMap<char, u128> = game
        .character_count
        .iter()
        .map(|x| (*x.0 as char, *x.1))
        .collect();
    println!("{:?}", as_chars);
    println!();
    println!("{:?}", game.frequencies);
    println!();
    println!("{} - {} = {}", most, least, most - least);
}

fn play_round(game: &mut Game) {
    let mut new_frequencies = HashMap::new();
    for (pair, freq) in game.frequencies.iter() {
        let b = *game.rules.get(&pair).unwrap();

        let new_pair_1 = Pair::from_bytes(pair.first, b);
        let new_pair_2 = Pair::from_bytes(b, pair.second);
        let old_val_1 = *new_frequencies.get(&new_pair_1).unwrap_or(&0);
        new_frequencies.insert(new_pair_1, old_val_1 + freq);

        let old_val_2 = *new_frequencies.get(&new_pair_2).unwrap_or(&0);
        new_frequencies.insert(new_pair_2, old_val_2 + freq);
        println!("Constructed {new_pair_1} and {new_pair_2} from {pair}");
        println!(
            "Bumped occurrences of {new_pair_1} from {old_val_1} to {}",
            old_val_1 + freq
        );
        println!(
            "Bumped occurrences of {new_pair_2} from {old_val_2} to {}",
            old_val_2 + freq
        );

        game.character_count.insert(
            b,
            *game.character_count.get(&b).unwrap_or(&0) + *freq as u128,
        );
    }

    game.frequencies = new_frequencies;
}

fn find_common_elements(character_count: &CharacterCount) -> (u128, u128) {
    let most_common = *character_count.values().max().unwrap();
    let least_common = *character_count.values().min().unwrap();

    (most_common, least_common)
}

fn increment_pair(frequencies: &mut Frequencies, pair: Pair) {
    frequencies.insert(pair, *frequencies.get(&pair).unwrap_or(&0) + 1);
}

fn parse_data() -> Game {
    let mut lines = DATA.lines().peekable();
    let initial_polymer = lines.next().unwrap().trim();

    let mut rules = HashMap::new();
    let mut frequencies = HashMap::new();
    for mut line in lines {
        line = line.trim();
        if line == "" {
            continue;
        }
        let mut split = line.split("->");
        let key = split.next().unwrap().trim();
        let pair = Pair::new(key);
        let value = split.next().unwrap().trim();
        let c = value.bytes().next().unwrap();
        rules.insert(pair, c);
    }

    for i in 1..initial_polymer.len() {
        let current_pair = &initial_polymer[i - 1..=i];
        let current_pair = Pair::new(current_pair);
        increment_pair(&mut frequencies, current_pair);
    }
    let mut character_count = HashMap::new();
    for b in initial_polymer.bytes() {
        character_count.insert(b, *character_count.get(&b).unwrap_or(&0) + 1);
    }
    Game::new(frequencies, rules, character_count)
}
