pub fn check(candidate: &str) -> bool {
  let mut store: Vec<char> = vec![];
  let ignored: [char; 2] = [' ', '-'];
  let mut is_isogram: bool = true;
  for letter in candidate.to_lowercase().chars() {
    if !ignored.contains(&letter) && store.contains(&letter) {
      is_isogram = false;
      break;
    }
    store.push(letter);
  }
  is_isogram
}
