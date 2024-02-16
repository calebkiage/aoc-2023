use std::ops::RangeFrom;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::{digit1, multispace0};
use nom::combinator::{map, map_res, opt, verify};
use nom::error::ParseError;
use nom::multi::fold_many1;
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::{
    AsChar, Compare, IResult, InputIter, InputLength, InputTake, InputTakeAtPosition, Parser, Slice,
};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws0<'a, F, I, O, E: ParseError<I>>(inner: F) -> impl Parser<I, O, E>
where
    F: Parser<I, O, E>,
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    delimited(multispace0, inner, multispace0)
}

/// Parse Game <id>:
fn game_id<I>(input: I) -> IResult<I, u64>
where
    I: Clone
        + Compare<&'static [u8]>
        + InputIter
        + InputLength
        + InputTake
        + InputTakeAtPosition
        + Slice<RangeFrom<usize>>,
    <I as InputIter>::Item: nom::AsChar + Clone,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    delimited(
        ws0(tag(b"Game".as_slice())),
        nom::character::complete::u64,
        tag(b":".as_slice()),
    )(input)
}

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

type ColorCount = (u64, Color);

type SubSet = [Option<ColorCount>; 3];

fn color<I>(input: I) -> IResult<I, Color>
where
    I: Clone + Compare<&'static [u8]> + InputTake,
{
    alt((
        map(tag(b"red".as_slice()), |_| Color::Red),
        map(tag(b"green".as_slice()), |_| Color::Green),
        map(tag(b"blue".as_slice()), |_| Color::Blue),
    ))(input)
}

// <num> <r|b|g>,
fn color_count<I>(input: I) -> IResult<I, ColorCount>
where
    I: Clone
        + Compare<&'static [u8]>
        + InputIter
        + InputLength
        + InputTake
        + InputTakeAtPosition
        + Slice<RangeFrom<usize>>,
    <I as InputIter>::Item: nom::AsChar + Clone,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    terminated(
        pair(ws0(nom::character::complete::u64), ws0(color)),
        opt(ws0(tag(b",".as_slice()))),
    )(input)
}

// <num> <r|g|b>,[<num> <r|g|b>,],[<num> <r|g|b>,]
fn subset<I>(input: I) -> IResult<I, SubSet>
where
    I: Clone
        + Compare<&'static [u8]>
        + InputIter
        + InputLength
        + InputTake
        + InputTakeAtPosition
        + Slice<RangeFrom<usize>>,
    <I as InputIter>::Item: nom::AsChar + Clone,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    map(
        verify(
            terminated(
                tuple((opt(color_count), opt(color_count), opt(color_count))),
                opt(ws0(tag(b";".as_slice()))),
            ),
            |(a, b, c)| a.is_some() || b.is_some() || c.is_some(),
        ),
        |(a, b, c)| [a, b, c],
    )(input)
}

#[derive(Debug, Default, PartialEq)]
pub struct Game {
    pub id: u64,
    // r, g, b
    pub handfuls: Vec<(u64, u64, u64)>,
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
pub fn parse_line(line: &[u8]) -> IResult<&[u8], Game> {
    map(
        pair(
            game_id,
            fold_many1(
                subset,
                || Game::default(),
                |mut game, subset| {
                    let mut r = 0;
                    let mut g = 0;
                    let mut b = 0;
                    for c in subset {
                        if let Some((count, color)) = c {
                            match color {
                                Color::Red => r += count,
                                Color::Green => g += count,
                                Color::Blue => b += count,
                            }
                        }
                    }
                    game.handfuls.push((r, g, b));
                    game
                },
            ),
        ),
        |(id, mut game)| {
            game.id = id;
            game
        },
    )(line)
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    mod color_count {
        use super::*;

        #[test]
        fn returns_color_with_count() {
            // With trailing comma
            let res = color_count(&b"23 red,"[..]);
            assert_eq!(res, Ok((&b""[..], (23, Color::Red))));
            // Without trailing comma
            let res = color_count(&b"28 blue"[..]);
            assert_eq!(res, Ok((&b""[..], (28, Color::Blue))));
            // With trailing semi-colon
            let res = color_count(&b"30 green;"[..]);
            assert_eq!(res, Ok((&b";"[..], (30, Color::Green))));
        }
    }

    mod subset {
        use super::*;

        #[test]
        fn returns_all_provided_colors() {
            let res = subset(&b"3 blue, 4 red;"[..]);
            assert_eq!(
                res,
                Ok((
                    &b""[..],
                    [Some((3, Color::Blue)), Some((4, Color::Red)), None]
                ))
            );
            let res = subset(&b"8 green, 6 blue, 20 red"[..]);
            assert_eq!(
                res,
                Ok((
                    &b""[..],
                    [
                        Some((8, Color::Green)),
                        Some((6, Color::Blue)),
                        Some((20, Color::Red))
                    ]
                ))
            );
        }

        #[test]
        fn fails_when_no_match() {
            let res = subset(&b""[..]);
            assert!(res.is_err());
        }
    }

    mod game_id {
        use super::*;

        #[test]
        fn produces_game_id() {
            let res = game_id(&b"Game 1: rest"[..]);
            assert_eq!(res, Ok((b" rest".as_slice(), 1)));
            let res = game_id(&b"Game 40: rest"[..]);
            assert_eq!(res, Ok((b" rest".as_slice(), 40)));
        }

        #[test]
        fn fails_on_invalid_input() {
            // Incomplete
            let res = game_id(&b"Game"[..]);
            assert!(res.is_err());
            let res = game_id(&b"Game 1"[..]);
            assert!(res.is_err());

            // Invalid
            let res = game_id(&b"invalid"[..]);
            assert!(res.is_err());
        }
    }

    mod parse_line {
        use super::*;

        #[test]
        fn produces_game() {
            let res = parse_line(&b"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"[..]);
            assert!(res.is_ok());
            let (rem, g) = res.unwrap();
            assert_eq!(rem, &b""[..],);
            assert_eq!(&g.handfuls[..], [(4, 0, 3), (1, 2, 6), (0, 2, 0),])
        }
    }
}
