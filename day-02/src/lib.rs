use std::borrow::Cow;

pub mod parser;

pub fn process_games<'a, F: Fn(&parser::Game) -> bool, F2: FnMut(&parser::Game)>(
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
