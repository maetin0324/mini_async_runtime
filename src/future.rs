// use std::task::Waker;

use std::future::Future;

pub struct CountFuture {
  count: u32,
  complete_count: u32,
}

impl CountFuture {
  pub fn new(complete_count: u32) -> Self {
    CountFuture {
      count: 0,
      complete_count,
    }
  }
}

impl Future for CountFuture {
  type Output = u32;

  fn poll(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
    let this = self.get_mut();
    this.count += 1;
    if this.count < this.complete_count {
      std::task::Poll::Pending
    } else {
      println!("CountFuture is ready with value: {}", this.count);
      std::task::Poll::Ready(this.count)
    }
  }
}


