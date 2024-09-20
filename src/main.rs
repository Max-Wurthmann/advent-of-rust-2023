use std::{collections::HashSet, ops::Deref, thread::current};

use indoc::indoc;

fn main() {}

#[test]
fn day1test() {
    let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    let out = day1(input);
    assert!(out == 142)
}

fn day1(input: &str) -> usize {
    let x = input.lines().map(|line| -> usize {
        let mut left: char = '!';
        for c in line.chars() {
            if c.is_numeric() {
                left = c;
                break;
            }
        }
        let mut right: char = '!';
        for c in line.chars().rev() {
            if c.is_numeric() {
                right = c;
                break;
            }
        }
        let number_str = format!("{left}{right}");
        return number_str.parse().unwrap();
    });

    return x.sum();
}

#[test]
fn vaid_game_test() {
    let games = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    // query encoded as rgb, negative values: omitt <=> no restriction
    let query = [12, 13, 14]; //rgb

    let expected: Vec<bool> = games.lines().map(|l| valid_game(l, query)).collect();

    assert_eq!(expected, vec![true, true, false, false, true])
}

fn valid_game(draws: &str, query: [i32; 3]) -> bool {
    // trim prefix
    let draws = match draws.split_once(':') {
        Some((_, str)) => str,
        None => panic!("not a valid draws str: '{}’", draws),
    };

    for draws in draws.split(';') {
        for color_count in draws.trim().split(',') {
            let (count, color) = color_count.trim().split_once(' ').unwrap();

            let count: i32 = count.parse().unwrap();
            let i = match color {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                c => panic!("Not a valid color {}", c),
            };

            if query[i] < count {
                return false;
            };
        }
    }

    return true;
}

#[test]
fn day2test() {
    let games = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    // query encoded as rgb, negative values: omitt <=> no restriction
    let query = [12, 13, 14];

    let result = day2(games, query);

    assert_eq!(result, 8);
}

fn day2(games: &str, query: [i32; 3]) -> i32 {
    let total = games
        .lines()
        .filter(|l| valid_game(l, query))
        .map(|l| -> i32 {
            let (str, _) = l.split_once(':').unwrap();
            let id: i32 = str.strip_prefix("Game ").unwrap().parse().unwrap();

            id
        })
        .reduce(|a, b| a + b)
        .unwrap();

    total
}

#[test]
fn day3test() {
    let input = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    let expected = 4361;
    let result = day3(input);

    println!("re:{result}, ex:{expected}");
    assert_eq!(expected, result);
}

fn day3(schema: &str) -> i32 {
    let mut sum = 0;

    let mut line_iter = schema.lines();

    let mut prev_line: Option<&str> = None;
    let mut curr_line: Option<&str> = line_iter.next();
    let mut next_line: Option<&str> = line_iter.next();

    while let Some(curr_str) = curr_line {
        let mut prev_chars = prev_line.map(|l| l.chars());
        let mut next_chars = next_line.map(|l| l.chars());

        let mut digit_buf = String::new();

        let mut symbol_flag = false;

        for curr_char in curr_str.chars() {
            let prev_char = match prev_chars {
                Some(ref mut iter) => iter.next(),
                None => None,
            };

            let next_char = match next_chars {
                Some(ref mut iter) => iter.next(),
                None => None,
            };

            let curr_symb =
                is_symbol(prev_char) || is_symbol(next_char) || is_symbol(Some(curr_char));

            if curr_char.is_numeric() {
                digit_buf.push(curr_char);
                symbol_flag = symbol_flag || curr_symb;
            } else {
                if (symbol_flag || curr_symb) && !digit_buf.is_empty() {
                    sum += digit_buf.parse::<i32>().unwrap();
                }

                symbol_flag = curr_symb;
                digit_buf = String::new();
            }
        }

        prev_line = curr_line;
        curr_line = next_line;
        next_line = line_iter.next();
    }

    sum
}

fn is_symbol(c_opt: Option<char>) -> bool {
    let c = match c_opt {
        Some(c) => c,
        None => return false,
    };

    let symbols = "$%&+-*#";

    symbols.contains(c)
}

#[test]
fn day4test() {
    let input = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    let expected = 13;
    let result = day4(input);

    assert_eq!(expected, result);
}

fn day4(scratch_cards: &str) -> i32 {
    scratch_cards
        .lines()
        .map(|line| -> &str {
            let (_, right) = line.split_once(':').unwrap();
            right
        })
        .map(|line| line.split_once('|').unwrap())
        .map(|(left, right)| {
            let winning_numbers: HashSet<i32> = left
                .split(' ')
                .filter_map(|str| str.parse::<i32>().ok())
                .collect();

            let drawn_numbers = right.split(' ').filter_map(|str| str.parse::<i32>().ok());

            let winning_count = drawn_numbers
                .filter(|val| winning_numbers.contains(val))
                .count();

            winning_count
        })
        .map(|count| {
            if count == 0 {
                0
            } else {
                2_i32.pow((count - 1).try_into().unwrap())
            }
        })
        .sum()
}

#[test]
fn day5test() {
    let seeds = [79, 14, 55, 13];

    let alamanac = indoc!(
        "
    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4
    "
    );

    assert_eq!(82, identify_location(alamanac, seeds[0]));
    assert_eq!(43, identify_location(alamanac, seeds[1]));
    assert_eq!(86, identify_location(alamanac, seeds[2]));
    assert_eq!(35, identify_location(alamanac, seeds[3]));

    assert_eq!(Some(35), day5(alamanac, &seeds));
}

fn identify_location(alamanac: &str, seed_nr: i32) -> i32 {
    let mut current_nr = seed_nr;
    let mut block_done_flag = false;

    for line in alamanac.lines() {
        let line = line.trim();

        if line.is_empty() {
            // new block reached
            block_done_flag = false;
            // skip empty lines
            continue;
        }

        if block_done_flag {
            // current block was already applied, continue to next block
            continue;
        };

        if let Some(c) = line.chars().next() {
            if !c.is_numeric() {
                // skip map name at start of block
                continue;
            }
        }

        let range_delim: Vec<i32> = line.split(' ').filter_map(|seq| seq.parse().ok()).collect();

        assert_eq!(
            3,
            range_delim.len(),
            "line '{line}' resulted in unexpected range_delim {range_delim:?}"
        );

        let dest_min = range_delim[0];
        let source_min = range_delim[1];
        let source_max = range_delim[1] + range_delim[2];
        if source_min <= current_nr && current_nr <= source_max {
            current_nr += dest_min - source_min;
        }
    }

    current_nr
}

fn day5(alamanac: &str, seed_numbers: &[i32]) -> Option<i32> {
    seed_numbers
        .iter()
        .map(|seed: &i32| identify_location(alamanac, *seed))
        .min()
}
