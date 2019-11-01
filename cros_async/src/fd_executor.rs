// Copyright 2019 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::os::unix::io::{AsRawFd, RawFd};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll};
use std::task::{RawWaker, RawWakerVTable, Waker};

use sys_util::PollContext;

thread_local!(static STATE: RefCell<InterfaceState> = RefCell::new(InterfaceState::new()));

/// Runs futures to completion on a single thread. Futures are allowed to block on file descriptors
/// only. Futures can only block on FDs becoming readable. `FdExecutor` is meant to be used where a
/// poll or select loop would be used otherwise.
pub struct FdExecutor {
    futures: Vec<(Pin<Box<dyn Future<Output = ()>>>, AtomicBool)>,
}

impl FdExecutor {
    /// Create a new executor.
    pub fn new() -> FdExecutor {
        FdExecutor {
            futures: Vec::new(),
        }
    }

    /// Appends the given future to the list of futures to run.
    /// These futures must return `()`. The futures added here are intended to driver side-effects
    /// only. Use `add_future` for top-level futures.
    pub fn add_future(&mut self, future: Pin<Box<dyn Future<Output = ()>>>) {
        self.futures.push((future, AtomicBool::new(true)));
    }

    /// Run the executor, this will consume the executor and only return once all the futures
    /// added to it have completed.
    pub fn run(mut self) {
        STATE.with(|state| {
            self.futures.append(&mut state.borrow_mut().new_futures);
        });

        loop {
            // This loop could use the unstable drain_filter() from Vec.
            for i in 0..self.futures.len() {
                let (fut, ready) = &mut self.futures[i];

                if !ready.load(Ordering::Relaxed) {
                    continue;
                }

                ready.store(false, Ordering::Relaxed);
                let raw_waker = unsafe { create_waker(ready as *const _ as *const _) };

                let waker = unsafe { Waker::from_raw(raw_waker) };
                let mut ctx = Context::from_waker(&waker);
                let f = fut.as_mut();
                match f.poll(&mut ctx) {
                    Poll::Pending => (),
                    Poll::Ready(()) => {
                        self.futures.remove(i);
                    }
                }
            }

            // Add any new futures to the list.
            let all_done = STATE.with(|state| {
                let mut state = state.borrow_mut();
                self.futures.append(&mut state.new_futures);

                if self.futures.is_empty() {
                    return true;
                }

                // Make sure there aren't any futures ready before sleeping.
                if !self
                    .futures
                    .iter()
                    .any(|(_fut, ready)| ready.load(Ordering::Relaxed))
                {
                    state.wait_wake_readable();
                }

                false
            });

            if all_done {
                return;
            }
        }
    }
}

/// Handles tracking the state of any futures blocked on FDs and allows adding a wake up request
/// from the poll funciton of a future.
struct InterfaceState {
    poll_ctx: PollContext<u64>,
    token_map: HashMap<u64, (SavedFd, Waker)>,
    next_token: u64,
    new_futures: Vec<(Pin<Box<dyn Future<Output = ()>>>, AtomicBool)>,
}

/// The interface for a future to interact with the executor that runs it.
/// Interfaces are provided to specify the FD to block on and for adding new futures to the
/// executor.
/// Used by futures who want to block until an FD becomes readable.
/// Keeps a list of FDs and associated wakers that will be woekn with `wake_by_ref` when the FD
/// becomes readable.
impl InterfaceState {
    /// Create an empty InterfaceState.
    pub fn new() -> InterfaceState {
        InterfaceState {
            poll_ctx: PollContext::new().unwrap(),
            token_map: HashMap::new(),
            next_token: 0,
            new_futures: Vec::new(),
        }
    }

    /// Waits until one of the FDs is readable and wakes the associated waker.
    pub fn wait_wake_readable(&mut self) {
        let events = self.poll_ctx.wait().unwrap();
        for e in events.iter_readable() {
            if let Some((fd, waker)) = self.token_map.remove(&e.token()) {
                self.poll_ctx.delete(&fd).unwrap();
                waker.wake_by_ref();
            }
        }
    }
}

/// Tells the waking system to wake `waker` when `fd` becomes readable.
pub fn add_waker(fd: &dyn AsRawFd, waker: Waker) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        while state.token_map.contains_key(&state.next_token) {
            state.next_token += 1;
        }
        state.poll_ctx.add(fd, state.next_token).unwrap();
        let next_token = state.next_token;
        state
            .token_map
            .insert(next_token, (SavedFd(fd.as_raw_fd()), waker));
    });
}

/// Adds a new top level future to the Executor.
/// These futures must return `()`. The futures added here are intended to driver side-effects
/// only. Use `add_future` for top-level futures.
pub fn add_future(future: Pin<Box<dyn Future<Output = ()>>>) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.new_futures.push((future, AtomicBool::new(true)));
    });
}

// Saved FD exists becaus RawFd doesn't impl AsRawFd.
struct SavedFd(RawFd);
impl AsRawFd for SavedFd {
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

// Boiler-plate for creating a waker with funciton pointers.
// This waker sets the atomic bool it is passed to true.
// The bool will be used by the executor to know which futures to poll
unsafe fn waker_drop(_: *const ()) {}
unsafe fn waker_wake(_: *const ()) {}
unsafe fn waker_wake_by_ref(data_ptr: *const ()) {
    println!("wake by ref");
    let bool_atomic_ptr = data_ptr as *const AtomicBool;
    let bool_atomic_ref = bool_atomic_ptr.as_ref().unwrap();
    bool_atomic_ref.store(true, Ordering::Relaxed);
}
unsafe fn waker_clone(data_ptr: *const ()) -> RawWaker {
    create_waker(data_ptr)
}

static WAKER_VTABLE: RawWakerVTable =
    RawWakerVTable::new(waker_clone, waker_wake, waker_wake_by_ref, waker_drop);

unsafe fn create_waker(data_ptr: *const ()) -> RawWaker {
    RawWaker::new(data_ptr, &WAKER_VTABLE)
}
