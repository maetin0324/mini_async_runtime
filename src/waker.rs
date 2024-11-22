use std::task::Waker;


// ダミーのWakerを作成する関数
pub fn dummy_waker() -> Waker {
  use std::task::{RawWaker, RawWakerVTable};

  fn no_op(_: *const ()) {}
  fn clone(_: *const ()) -> RawWaker {
      dummy_raw_waker()
  }

  fn dummy_raw_waker() -> RawWaker {
      RawWaker::new(std::ptr::null(), &RawWakerVTable::new(clone, no_op, no_op, no_op))
  }

  unsafe { Waker::from_raw(dummy_raw_waker()) }
}