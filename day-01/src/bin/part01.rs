use std::time::Instant;

fn main() {
    let start = Instant::now();
    // Assume ASCII text
    let input = include_bytes!("../../data/input.txt");
    let result: usize = input
        // 51600ns
        .split(|b| b == &b'\n')
        .filter_map(find_num_pair_in_line)
        .sum();
    println!("{result:#?} in {}ns", start.elapsed().as_nanos())
}

#[inline]
fn find_first_digit_in_line(line: &[u8]) -> Option<&u8> {
    line.iter().find(|n| n.is_ascii_digit())
}

#[inline]
fn find_last_digit_in_line(line: &[u8]) -> Option<&u8> {
    line.iter().rfind(|n| n.is_ascii_digit())
}

fn find_num_pair_in_line(line: &[u8]) -> Option<usize> {
    if let (Some(l), Some(r)) = (
        find_first_digit_in_line(line),
        find_last_digit_in_line(line),
    ) {
        let ones = (r - b'0') as usize;
        let tens = ((l - b'0') * 10) as usize;
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
}
