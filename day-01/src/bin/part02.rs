use aho_corasick::AhoCorasick;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    // Assume ASCII text
    let input = include_bytes!("../../data/input.txt");
    let result: usize = input
        // 217194700ns
        .split(|b| b == &b'\n')
        .filter_map(find_num_pair_in_line)
        .sum();
    println!("{result:#?} in {}ns", start.elapsed().as_nanos())
}

const DIGITS: &[&[u8]] = [
    b"zero".as_slice(),
    b"one".as_slice(),
    b"two".as_slice(),
    b"three".as_slice(),
    b"four".as_slice(),
    b"five".as_slice(),
    b"six".as_slice(),
    b"seven".as_slice(),
    b"eight".as_slice(),
    b"nine".as_slice(),
]
.as_slice();

#[inline]
fn find_first_digit_in_line(line: &[u8]) -> Option<u8> {
    let ac = AhoCorasick::builder().build(DIGITS).unwrap();
    let m = ac
        .try_find(line)
        .ok()
        .flatten()
        .map(|m| (m.start(), m.pattern().as_u32() as u8));
    let found = line.iter().enumerate().find(|(_, b)| b.is_ascii_digit());

    match (m, found) {
        (Some((ia, a)), Some((ib, b))) => {
            if ia < ib {
                Some(a)
            } else {
                Some(ascii_digit_to_num(*b))
            }
        }
        (Some((_, a)), None) => Some(a),
        (None, Some((_, b))) => Some(ascii_digit_to_num(*b)),
        (None, None) => None,
    }
}

#[inline]
fn find_last_digit_in_line(line: &[u8]) -> Option<u8> {
    let ac = AhoCorasick::builder().build(DIGITS).unwrap();
    let m = ac
        .try_find_iter(line)
        .map(|f| f.last())
        .ok()
        .flatten()
        .map(|m| (m.start(), m.pattern().as_u32() as u8));
    let found = line.iter().enumerate().rfind(|(_, b)| b.is_ascii_digit());

    match (m, found) {
        (Some((ia, a)), Some((ib, b))) => {
            if ia < ib {
                Some(ascii_digit_to_num(*b))
            } else {
                Some(a)
            }
        }
        (Some((_, a)), None) => Some(a),
        (None, Some((_, b))) => Some(ascii_digit_to_num(*b)),
        (None, None) => None,
    }
}

fn find_num_pair_in_line(line: &[u8]) -> Option<usize> {
    if let (Some(l), Some(r)) = (
        find_first_digit_in_line(line),
        find_last_digit_in_line(line),
    ) {
        let ones = (r) as usize;
        let tens = (l * 10) as usize;
        let num = tens + ones;
        Some(num)
    } else {
        None
    }
}

#[inline]
fn ascii_digit_to_num(digit: u8) -> u8 {
    debug_assert!(digit.is_ascii_digit());
    digit - b'0'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_num_pair_in_line_works_when_no_num() {
        let data = b"no_number";
        let res = find_num_pair_in_line(data);
        assert_eq!(res, None);
    }

    #[test]
    fn test_find_num_pair_in_line_works_when_single_num() {
        let data = b"1";
        let res = find_num_pair_in_line(data);
        assert_eq!(res, Some(11));
        let data = b"num1withtext";
        let res = find_num_pair_in_line(data);
        assert_eq!(res, Some(11));
    }

    #[test]
    fn test_find_num_pair_in_line_works_when_multiple_nums() {
        let data = b"123";
        let res = find_num_pair_in_line(data);
        assert_eq!(res, Some(13));
        let data = b"num1with2and3";
        let res = find_num_pair_in_line(data);
        assert_eq!(res, Some(13));
    }

    #[test]
    fn test_find_digit_in_line_works_with_words() {
        let data = b"two1nine";
        let res = find_first_digit_in_line(data);
        assert_eq!(res, Some(2));
        let res = find_last_digit_in_line(data);
        assert_eq!(res, Some(9));
        let data = b"no_num";
        let res = find_first_digit_in_line(data);
        assert_eq!(res, None);
        let res = find_last_digit_in_line(data);
        assert_eq!(res, None);
    }
    #[test]
    fn test_find_digit_in_line_works_with_numbers() {
        let data = b"x1zero";
        let res = find_first_digit_in_line(data);
        assert_eq!(res, Some(1));
        let res = find_last_digit_in_line(data);
        assert_eq!(res, Some(0));
    }
}
