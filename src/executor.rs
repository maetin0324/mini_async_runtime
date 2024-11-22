use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::waker::dummy_waker;

pub struct CountExecutor<T> 
where
  T: 'static + std::fmt::Debug,
{
  run_queue: Vec<Pin<Box<dyn Future<Output = T>>>>,
}

impl<T> CountExecutor<T> 
where
  T: 'static + std::fmt::Debug,
{
  pub fn new() -> Self {
    CountExecutor {
      run_queue: Vec::new(),
    }
  }

  pub fn spawn(&mut self, future: Pin<Box<dyn Future<Output = T>>>) {
    self.run_queue.push(future);
  }

  pub fn run(&mut self, f: impl Future<Output = T> + 'static) {
    let waker = dummy_waker();
    let mut context = Context::from_waker(&waker);

    self.spawn(Box::pin(f));

    while !self.run_queue.is_empty() {
      let mut future = self.run_queue.remove(0);
      match Future::poll(Pin::as_mut(&mut future), &mut context) {
        Poll::Pending => {
          self.run_queue.push(future);
        }
        Poll::Ready(value) => {
          println!("Future is ready with value: {:?}", value);
        }
      }
    }
  }
}
