use crate::Command;
use crate::Fan;

pub struct FanCommand<'a> {
  pub fan: &'a Fan
}

impl<'a> Command for FanCommand<'a> {
  fn execute(&self) {
    self.fan.blow_wind();
  }
}