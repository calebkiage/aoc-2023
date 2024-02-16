use std::{borrow::Cow, io::BufRead as _};

use day_02::process_games;

fn main() {
    let input = include_bytes!("../../data/input.txt");
    let result = cubes_power(input).unwrap();
    eprintln!("{result}");
}

fn cubes_power(input: &[u8]) -> Result<u64, Cow<'static, str>> {
    let mut sum_power = 0;
    process_games(
        input.lines(),
        |_| true,
        |game| {
            let (mut r, mut g, mut b) = (0, 0, 0);
            for (r0, g0, b0) in &game.handfuls {
                if *r0 > r {
                    r = *r0;
                }
                if *g0 > g {
                    g = *g0;
                }

                if *b0 > b {
                    b = *b0;
                }
            }
            let pow = r * g * b;
            sum_power += pow;
        },
    )?;
    Ok(sum_power)
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

        let pow = cubes_power(INPUT).unwrap();
        assert_eq!(pow, 2286);
    }
}
