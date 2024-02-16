mod parser;

use std::{borrow::Cow, io::BufRead as _};

fn main() {
    let input = include_bytes!("../data/input.txt");
    let result = sum_possible_game_ids(input, 12, 13, 14).unwrap();
    eprintln!("{result}");
}

fn sum_possible_game_ids(
    input: &[u8],
    reds: u64,
    greens: u64,
    blues: u64,
) -> Result<u64, Cow<'static, str>> {
    let mut sum_ids = 0;
    possible_games(
        input.lines(),
        |g| {
            for (r, g, b) in &g.handfuls {
                if *r > reds || *g > greens || *b > blues {
                    return false;
                }
            }
            true
        },
        |g| sum_ids += g.id,
    )?;
    Ok(sum_ids)
}

fn possible_games<'a, F: Fn(&parser::Game) -> bool, F2: FnMut(&parser::Game)>(
    lines: std::io::Lines<impl std::io::BufRead>,
    predicate: F,
    mut operation: F2,
) -> Result<(), Cow<'static, str>> {
    for line in lines.filter(|l| match l {
        Ok(l) => !l.trim().is_empty(),
        Err(_) => true,
    }) {
        let line = line.map_err(|e| Cow::Owned(format!("could not read line: {e}")))?;
        let (_, game) = parser::parse_line(line.trim().as_bytes())
            .map_err(|_| Cow::Borrowed("could not parse line"))?;
        if predicate(&game) {
            operation(&game)
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        const INPUT: &[u8] = br#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;

        let sum_ids = sum_possible_game_ids(INPUT, 12, 13, 14).unwrap();
        assert_eq!(sum_ids, 8);
    }
}
