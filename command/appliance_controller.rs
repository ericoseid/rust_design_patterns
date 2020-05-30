use crate::Command;

pub struct ApplianceController<'a> {
  pub appliances: Vec<&'a dyn Command>
}

impl<'a> ApplianceController<'a> {
  pub fn set_command(&mut self, appliance: &'a dyn Command) {
    self.appliances.push(appliance);
  }

  pub fn use_appliance(&self, appliance_index: usize) {
    self.appliances[appliance_index].execute();
  }
}