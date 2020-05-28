use crate::Beverage;

pub struct Americano {}

impl Beverage for Americano {
  fn get_cost(&self) -> i32 {
    return 20;
  }
}