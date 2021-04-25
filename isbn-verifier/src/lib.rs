/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let safe_isbn = isbn.replace("-", "");
    if safe_isbn.len() != 10 {
        return false;
    }

    let processed_isbn = safe_isbn
        .chars()
        .enumerate()
        .filter(|(i, x)| x.is_ascii_digit() || (x == &'X' && *i == safe_isbn.len() - 1))
        .collect::<Vec<(usize, char)>>();

    if processed_isbn.len() != 10 {
        return false;
    }

    let sum: i32 = processed_isbn
        .iter()
        .map(|(i, x)| {
            let safe_x = if x == &'X' {
                10
            } else {
                x.to_digit(10).unwrap() as i32
            };
            safe_x * (10 - *i as i32)
        })
        .sum();
    sum % 11 == 0
}
