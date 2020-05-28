use crate::Observer;
use crate::Subject;

pub struct WeatherObject {
  pub displays: Vec<Box<dyn Observer>>,
}

impl Subject for WeatherObject {
  fn register_observer(&mut self, observer: impl Observer + 'static) {
    self.displays.push(Box::from(observer));
  }

  fn notify_observers(&self) {
    for display in self.displays.iter() {
      display.update();
    }
  }
}
