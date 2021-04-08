use std::collections::HashMap;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
  let mut max_by_row = HashMap::new();
  let mut min_by_column = HashMap::new();

  for (ri, row) in input.iter().enumerate() {
    for (ci, elem) in row.iter().enumerate() {
      max_by_row
        .entry(ri)
        .and_modify(|max| {
          if *max < *elem {
            *max = *elem;
          }
        })
        .or_insert(*elem);

      min_by_column
        .entry(ci)
        .and_modify(|min| {
          if *min > *elem {
            *min = *elem
          }
        })
        .or_insert(*elem);
    }
  }

  let mut result = vec![];
  for (ri, row) in input.iter().enumerate() {
    for (ci, num) in row.iter().enumerate() {
      if num == max_by_row.get(&ri).unwrap() && num == min_by_column.get(&ci).unwrap() {
        result.push((ri, ci));
      }
    }
  }
  result
}
