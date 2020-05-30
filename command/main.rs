mod command;
mod fan;
mod fan_command;
mod light;
mod light_command;
mod appliance_controller;

use command::Command;
use fan::Fan;
use fan_command::FanCommand;
use light::Light;
use light_command::LightCommand;
use appliance_controller::ApplianceController;

fn main() {
  let f = Fan {};
  let f_com = FanCommand { fan: &f };

  let l = Light {};
  let l_com = LightCommand { light: &l };

  let mut controller = ApplianceController {
    appliances : Vec::new()
  };

  controller.set_command(&f_com);
  controller.set_command(&l_com);

  controller.use_appliance(0);
  controller.use_appliance(1);
}