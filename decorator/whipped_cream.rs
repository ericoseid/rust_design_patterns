use crate::Beverage;
use crate::Condiment;

pub struct WhippedCream<'a> {
  pub bev: &'a dyn Beverage,
}

impl<'a> Beverage for WhippedCream<'a> {
  fn get_cost(&self) -> i32 {
    return 5 + self.bev.get_cost();
  }
}

impl<'a> Condiment for WhippedCream<'a> {}