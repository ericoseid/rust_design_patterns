use crate::Beverage;

pub struct Espresso {}

impl Beverage for Espresso {
  fn get_cost(&self) -> i32 {
    return 10;
  }
}