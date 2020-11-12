fn count_occupied_cell(c: &char) -> usize {
  if c.to_string().len() > 1 {
    2
  } else {
    1
  }
}

pub fn cell_length(s: &str) -> usize {
  s.chars().fold(0, |acc, c| acc + count_occupied_cell(&c))
}

pub fn head_str(s: &str, ncell: usize) -> String {
  let mut acc = 0;
  s.chars()
    .filter(|c| {
      acc += count_occupied_cell(c);
      acc <= ncell
    })
    .collect()
}
