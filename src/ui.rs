use std::io::{self, ErrorKind};

pub fn scan() -> io::Result<String> {
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer)?;
  Ok(buffer)
}

pub fn prompt(text: &str) -> io::Result<String> {
  println!("{}", text);
  scan()
}

pub fn yesno(text: &str) -> io::Result<bool> {
  println!("{} (y/n)", text);
  let response: String = (scan()?).trim().to_uppercase();
  Ok(response == "Y" || response == "YES")
}

pub fn selection(text: &str, range: R) -> io::Result<u8>
  where R: RangeBounds<u8>
{
  println!("{}", text);
  println!("Please choose one:");
  let res = (scan()?).trim().parse::<u8>();
  if res.is_ok() {
    res
  } else {
    Err(io::Error(ErrorKind::InvalidInput, "Invalid Selection"))
  }
}
