#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num < 1 {
        return None;
    } else if num == 1 {
        return Some(Classification::Deficient);
    }
    let num_factors_sum = (1..num as i64)
        .filter(|i| num as i64 % i == 0)
        .collect::<Vec<i64>>()
        .iter()
        .sum::<i64>();
    if num_factors_sum == num as i64 {
        Some(Classification::Perfect)
    } else if num_factors_sum < num as i64 {
        Some(Classification::Deficient)
    } else if num_factors_sum > num as i64 {
        Some(Classification::Abundant)
    } else {
        None
    }
}
