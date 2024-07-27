
use std::thread;
use thread_waker::waker;

#[test]
fn should_not_drop_on_wake_by_ref() {
    let waker = waker(thread::current());

    let waker2 = waker.clone();

    waker.wake();
    waker2.wake_by_ref();
    drop(waker2);
}
