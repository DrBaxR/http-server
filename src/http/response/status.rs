// todo: add more statuses
pub enum ResponseStatus {
  Ok,
  NotFound,
}

impl ResponseStatus {
  pub fn to_string(&self) -> String {
    match self {
        Self::Ok => String::from("200 OK"),
        Self::NotFound => String::from("404 Not Found"),
    }
  }
}
