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
