# thread-waker

[![Rust](https://github.com/DoumanAsh/thread-waker/actions/workflows/rust.yml/badge.svg)](https://github.com/DoumanAsh/thread-waker/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/thread-waker.svg)](https://crates.io/crates/thread-waker)
[![Documentation](https://docs.rs/thread-waker/badge.svg)](https://docs.rs/crate/thread-waker/)

Waker implementation using current thread token.

This is useful to work with futures without actually employing runtime

## Usage

```rust
use core::{time, task};
use std::thread;

use thread_waker::waker;

fn my_future(waker: task::Waker) {
    thread::sleep(time::Duration::from_millis(250));
    waker.wake();
}

let waker = waker(thread::current());

for _ in 0..4 {
    let waker = waker.clone();
    thread::spawn(move || my_future(waker));
    thread::park();
}

println!("I'm done!");
```
