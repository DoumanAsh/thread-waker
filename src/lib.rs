//!Waker implementation using current thread token.
//!
//!This is useful to work with futures without actually employing runtime
//!
//!## Usage
//!
//!```rust
//!use core::{time, task};
//!use std::thread;
//!
//!use thread_waker::waker;
//!
//!fn my_future(waker: task::Waker) {
//!    thread::sleep(time::Duration::from_millis(250));
//!    waker.wake();
//!}
//!
//!let waker = waker(thread::current());
//!
//!for _ in 0..4 {
//!    let waker = waker.clone();
//!    thread::spawn(move || my_future(waker));
//!    thread::park();
//!}
//!
//!println!("I'm done!");
//!```

#![warn(missing_docs)]
#![allow(clippy::style)]
// Dudue, do you think I'm retard not to know what I transmute?
#![allow(clippy::missing_transmute_annotations)]

use core::{task, mem};
use std::thread::Thread;

const VTABLE: task::RawWakerVTable = task::RawWakerVTable::new(clone, wake, wake_by_ref, on_drop);

unsafe fn on_drop(thread: *const ()) {
    let thread: Thread = mem::transmute(thread);
    drop(thread);
}

unsafe fn clone(thread: *const()) -> task::RawWaker {
    //Thread handle is just simple Arc pointer to cloning it is cheap and efficient
    //but we need to make sure to forget current thread, otherwise it is no clone
    //as Clone callback is done via reference
    let thread: Thread = mem::transmute(thread);
    let new_ptr = mem::transmute(thread.clone());
    mem::forget(thread);
    task::RawWaker::new(new_ptr, &VTABLE)
}

unsafe fn wake(thread: *const ()) {
    let thread: Thread = mem::transmute(thread);
    thread.unpark();
}

unsafe fn wake_by_ref(thread: *const ()) {
    let thread: Thread = mem::transmute(thread);
    thread.unpark();
    //wake_by_ref should not consume self
    mem::forget(thread);
}

#[inline(always)]
///Creates waker from thread handle
pub fn waker(thread: Thread) -> task::Waker {
    unsafe {
        task::Waker::from_raw(task::RawWaker::new(mem::transmute(thread), &VTABLE))
    }
}
