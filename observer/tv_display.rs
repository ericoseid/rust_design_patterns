use crate::Observer;

pub struct TvDisplay {}

impl Observer for TvDisplay {
  fn update(&self) {
    println!("Update TV display");
  }
}