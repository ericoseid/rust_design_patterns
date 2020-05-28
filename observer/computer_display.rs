use crate::Observer;

pub struct ComputerDisplay {}

impl Observer for ComputerDisplay {
  fn update(&self) {
    println!("Update computer display");
  }
}