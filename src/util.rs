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

pub enum SizeUnit {
  B,
  KB,
  MB,
  GB,
}

impl ToString for SizeUnit {
  fn to_string(&self) -> String {
    let s = match self {
      SizeUnit::B => "B",
      SizeUnit::KB => "KB",
      SizeUnit::MB => "MB",
      SizeUnit::GB => "GB",
    };
    s.to_string()
  }
}

impl SizeUnit {
  fn next_unit(&self) -> Option<SizeUnit> {
    match self {
      SizeUnit::B => Some(SizeUnit::KB),
      SizeUnit::KB => Some(SizeUnit::MB),
      SizeUnit::MB => Some(SizeUnit::GB),
      SizeUnit::GB => None,
    }
  }
}

pub fn size_of(size: usize, unit: SizeUnit) -> String {
  let div_size = size / 1024;
  if let Some(next_unit) = unit.next_unit() {
    if div_size > 1 {
      size_of(div_size, next_unit)
    } else {
      size.to_string() + &unit.to_string()
    }
  } else {
    size.to_string() + &unit.to_string()
  }
}
