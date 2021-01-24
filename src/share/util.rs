use async_trait::async_trait;
use chrono::{DateTime, Datelike, TimeZone, Timelike};

pub const DATE_FORMAT_CHAR_LENGHT: usize = 14; // e.g. 20 12 25 18:00 (14 chars)
pub const FILESIZE_FORMAT_CHAR_LENGTH: usize = 6; // e.g. 123.4K (6 chars)

pub type DisplayableFileData = String;

#[async_trait]
pub trait FormatDisplay {
  async fn format_display(&self, with_metadata: bool) -> DisplayableFileData;
}

pub fn format_datetime<T: TimeZone>(datetime: &DateTime<T>) -> String {
  let date = datetime.date();

  let year = date.year().to_string().get(2..).unwrap().to_string();

  let month = if date.month() >= 10 {
    date.month().to_string()
  } else {
    format!("0{}", date.month())
  };

  let day = if date.day() >= 10 {
    date.day().to_string()
  } else {
    format!("0{}", date.day())
  };

  let time = datetime.time();

  let hour = if time.hour() >= 10 {
    time.hour().to_string()
  } else {
    format!("0{}", time.hour())
  };

  let minute = if time.minute() >= 10 {
    time.minute().to_string()
  } else {
    format!("0{}", time.minute())
  };

  format!("{} {} {} {}:{}", year, month, day, hour, minute)
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
      SizeUnit::KB => "K",
      SizeUnit::MB => "M",
      SizeUnit::GB => "G",
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

pub fn size_of(size: f64, unit: SizeUnit) -> String {
  let div_size = size / 1024.;
  unit.next_unit().map_or_else(
    || format!("{:.1}", size) + &unit.to_string(),
    |nu| {
      if div_size < 1. {
        return format!("{:.1}", size) + &unit.to_string();
      }
      size_of(div_size, nu)
    },
  )
}

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

pub fn padding_left_until(base: &str, maxchars: usize) -> String {
  let nchars = base.len();
  if nchars >= maxchars {
    return base.to_string();
  }
  " ".repeat(maxchars - nchars) + base
}
