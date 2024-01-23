use std::time::Instant;

fn main() {
    let start = Instant::now();
    // Assume ASCII text
    let input = include_str!("../data/input.txt");
    let result: usize = input
        // 2376600ns
        .split(|b| b == '\n')
        .filter_map(find_num_pair_in_line)
        .sum();
    println!("{result:#?} in {}ns", start.elapsed().as_nanos())
}

const DIGITS: &[&str] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2",
    "3", "4", "5", "6", "7", "8", "9",
]
.as_slice();

#[inline]
fn find_first_and_last_digit_in_line(line: &str) -> Option<(u8, u8)> {
    let first = DIGITS
        .iter()
        .enumerate()
        .filter_map(|(i, d)| line.find(*d).map(|start| ((i % 10) as u8, start)))
        .min_by_key(|(_, start)| *start)
        .map(|(i, _)| i);
    let last = DIGITS
        .iter()
        .enumerate()
        .filter_map(|(i, d)| line.rfind(*d).map(|start| ((i % 10) as u8, start)))
        .max_by_key(|(_, start)| *start)
        .map(|(i, _)| i);
    if let (Some(a), Some(b)) = (first, last) {
        Some((a, b))
    } else {
        None
    }
}

#[inline]
fn find_num_pair_in_line(line: &str) -> Option<usize> {
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
        let data = "no_number";
        let res = find_num_pair_in_line(data);
        assert_eq!(res, None);
    }

    #[test]
    fn test_find_num_pair_in_line_works_when_single_num() {
        let data = "1";
        let res = find_num_pair_in_line(data);
        assert_eq!(res, Some(11));
        let data = "num1withtext";
        let res = find_num_pair_in_line(data);
        assert_eq!(res, Some(11));
    }

    #[test]
    fn test_find_num_pair_in_line_works_when_multiple_nums() {
        let data = "123";
        let res = find_num_pair_in_line(data);
        assert_eq!(res, Some(13));
        let data = "num1with2and3";
        let res = find_num_pair_in_line(data);
        assert_eq!(res, Some(13));
    }

    #[test]
    fn test_find_digit_in_line_works_with_words() {
        let data = "two1nine";
        let res = find_first_and_last_digit_in_line(data);
        assert_eq!(res, Some((2, 9)));
        let data = "no_num";
        let res = find_first_and_last_digit_in_line(data);
        assert_eq!(res, None);
    }
    #[test]
    fn test_find_digit_in_line_works_with_numbers() {
        let data = "x1zero";
        let res = find_first_and_last_digit_in_line(data);
        assert_eq!(res, Some((1, 0)));
    }
}
