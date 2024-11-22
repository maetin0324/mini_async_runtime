// use std::task::Waker;
use std::{future::Future, pin::Pin, task::{Context, Poll}};

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

  fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
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

pub fn count_future(complete_count: u32) -> CountFuture {
  CountFuture::new(complete_count)
}

pub struct HogeFuture {
  state: u8,
}

impl Future for HogeFuture {
  type Output = u32;

  fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
      let this = self.get_mut();
      match this.state {
          0 => {
              this.state = 1;
              Poll::Ready(42)
          }
          _ => panic!("Future polled after completion"),
      }
  }
}

pub fn hoge() -> HogeFuture {
  HogeFuture { state: 0 }
}



