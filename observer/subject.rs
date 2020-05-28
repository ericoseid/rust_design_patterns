use crate::Observer;

pub trait Subject {
  fn register_observer(&mut self, observer: impl Observer + 'static);
  fn notify_observers(&self);
}
