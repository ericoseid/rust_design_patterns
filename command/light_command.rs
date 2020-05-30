use crate::Command;
use crate::Light;

pub struct LightCommand<'a> {
  pub light: &'a Light
}

impl<'a> Command for LightCommand<'a> {
  fn execute(&self) {
    self.light.turn_on();
  }
}