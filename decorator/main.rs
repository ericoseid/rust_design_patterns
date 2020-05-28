mod beverage;
mod espresso;
mod americano;
mod condiment;
mod whipped_cream;

use beverage::Beverage;
use espresso::Espresso;
use americano::Americano;
use condiment::Condiment;
use whipped_cream::WhippedCream;

fn main() {

  //10
  let e = Espresso {};
  //20
  let a = Americano {};
  //25
  let americano_with_cream = WhippedCream {
    bev: &a
  };

  get_bev_cost(&e);
  get_bev_cost(&a);
  get_bev_cost(&americano_with_cream);
  get_condiment_cost(&americano_with_cream);
}

fn get_bev_cost(bev: &impl Beverage) {
  println!("{}", bev.get_cost());
}

fn get_condiment_cost(bev: &impl Condiment) {
  println!("{}", bev.get_cost());
}