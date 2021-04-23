pub fn encode(n: u64) -> String {
    match n {
        0 => String::from("zero"),
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        15 => String::from("fifteen"),
        18 => String::from("eighteen"),
        14..=19 => encode(n % 10) + "teen",
        20 => String::from("twenty"),
        30 => String::from("thirty"),
        40 => String::from("forty"),
        50 => String::from("fifty"),
        80 => String::from("eighty"),
        60..=90 if n % 10 == 0 => encode(n / 10) + "ty",
        21..=99 => encode(n - n % 10) + "-" + &encode(n % 10),
        100..=999 => encode_hundred_or_above(n, 100, "hundred"),
        1000..=999_999 => encode_hundred_or_above(n, 1000, "thousand"),
        1_000_000..=999_999_999 => encode_hundred_or_above(n, 1_000_000, "million"),
        1_000_000_000..=999_999_999_999 => encode_hundred_or_above(n, 1_000_000_000, "billion"),
        1_000_000_000_000..=999_999_999_999_999 => {
            encode_hundred_or_above(n, 1_000_000_000_000, "trillion")
        }
        1_000_000_000_000_000..=999_999_999_999_999_999 => {
            encode_hundred_or_above(n, 1_000_000_000_000_000, "quadrillion")
        }
        _ => encode_hundred_or_above(n, 1_000_000_000_000_000_000, "quintillion"),
    }
}

pub fn encode_hundred_or_above(n: u64, divisor: u64, stack: &str) -> String {
    if n % divisor == 0 {
        encode(n / divisor) + " " + stack
    } else {
        encode(n - n % divisor) + " " + &encode(n % divisor)
    }
}
