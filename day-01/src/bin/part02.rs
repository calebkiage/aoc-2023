use aho_corasick::AhoCorasick;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    // Assume ASCII text
    let input = include_bytes!("../../data/input.txt");
    let result: usize = input
        // 217194700ns
        // 70038700ns
        // 100163400ns
        // 63643900ns
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
    b"0".as_slice(),
    b"1".as_slice(),
    b"2".as_slice(),
    b"3".as_slice(),
    b"4".as_slice(),
    b"5".as_slice(),
    b"6".as_slice(),
    b"7".as_slice(),
    b"8".as_slice(),
    b"9".as_slice(),
]
.as_slice();

#[inline]
fn find_first_and_last_digit_in_line(line: &[u8]) -> Option<(u8, u8)> {
    let ac = AhoCorasick::builder().build(DIGITS).unwrap();
    ac.try_find_iter(line)
        .map(|mut f| {
            let next = f.next();
            let last = f.last().or(next);
            if let (Some(f), Some(l)) = (next, last) {
                Some((f, l))
            } else {
                None
            }
        })
        .ok()
        .flatten()
        .map(|(m0, m1)| {
            (
                (m0.pattern().as_u32() % 10) as u8,
                (m1.pattern().as_u32() % 10) as u8,
            )
        })
}

#[inline]
fn find_num_pair_in_line(line: &[u8]) -> Option<usize> {
    if let Some((l, r)) = find_first_and_last_digit_in_line(line) {
        let ones = (r) as usize;
        let tens = (l * 10) as usize;
        let num = tens + ones;
        Some(num)
    } else {
        None
    }
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
        let res = find_first_and_last_digit_in_line(data);
        assert_eq!(res, Some((2, 9)));
        let data = b"no_num";
        let res = find_first_and_last_digit_in_line(data);
        assert_eq!(res, None);
    }
    #[test]
    fn test_find_digit_in_line_works_with_numbers() {
        let data = b"x1zero";
        let res = find_first_and_last_digit_in_line(data);
        assert_eq!(res, Some((1, 0)));
    }
}
