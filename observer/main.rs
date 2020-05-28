mod subject;
mod observer;
mod weather_object;
mod computer_display;
mod tv_display;

use subject::Subject;
use observer::Observer;
use weather_object::WeatherObject;
use computer_display::ComputerDisplay;
use tv_display::TvDisplay;

fn main() {
  let mut station = WeatherObject {
    displays: Vec::new(),
  };

  let display1 = ComputerDisplay {};
  let display2 = TvDisplay {};
 
  station.register_observer(display1);
  station.register_observer(display2);

  station.notify_observers();

  println!("OBSERVER PATTERN!");
}