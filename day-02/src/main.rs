use std::{borrow::Cow, io::BufRead as _};

use day_02::process_games;

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
    process_games(
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
