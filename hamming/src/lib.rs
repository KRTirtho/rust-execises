/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    // edge cases
    if s1.len() != s2.len() {
        return None;
    } else if s1 == s2 {
        return Some(0);
    }
    let mut distance = 0;
    for (index, helix) in s1.chars().enumerate() {
        if helix != s2.chars().nth(index).unwrap_or_default() {
          distance+=1;
        }
    }
    Some(distance)
}
