// Copyright 2019 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! An Executor to be used to driver file descriptor based futures to completion.
//!
//! # Example of starting the framework and running a future:
//!
//! ```
//! async fn my_async() {
//!     futures::future::ready(1).await;
//! }
//!
//! let mut ex = cros_async::FdExecutor::new();
//! ex.add_future(Box::pin(my_async()));
//! ex.run();
//! ```
//!
//! When building futures to be run in an `FdExecutor` framework, use the following helper functions
//! to perform common tasks:
//!
//! `add_read_waker` - Used to associate a provided FD becoming readable with the future being
//! woken.
//! `add_write_waker` - Used to associate a provided FD becoming writable with the future being
//! woken.
//! `add_future` - Used to add a new future to the top-level list of futures running.
//!

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::future::Future;
use std::os::unix::io::{AsRawFd, RawFd};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll};
use std::task::{RawWaker, RawWakerVTable, Waker};

use sys_util::{PollContext, WatchingEvents};

// Temporary holding areas for things added to the executor.
// File descriptor wakers that are added during poll calls.
thread_local!(static FDS: RefCell<Vec<(SavedFd, Waker, WatchingEvents)>> = RefCell::new(Vec::new()));
// Top level futures that are added during poll calls.
thread_local!(static FUTURES: RefCell<Vec<(Pin<Box<dyn Future<Output = ()>>>, AtomicBool)>> = RefCell::new(Vec::new()));

/// Tells the waking system to wake `waker` when `fd` becomes readable.
pub fn add_read_waker(fd: &dyn AsRawFd, waker: Waker) {
    FDS.with(|new_fds| {
        let mut new_fds = new_fds.borrow_mut();
        new_fds.push((
            SavedFd(fd.as_raw_fd()),
            waker,
            WatchingEvents::empty().set_read(),
        ));
    });
}

/// Tells the waking system to wake `waker` when `fd` becomes writable.
pub fn add_write_waker(fd: &dyn AsRawFd, waker: Waker) {
    FDS.with(|new_fds| {
        let mut new_fds = new_fds.borrow_mut();
        new_fds.push((
            SavedFd(fd.as_raw_fd()),
            waker,
            WatchingEvents::empty().set_write(),
        ));
    });
}

/// Adds a new top level future to the Executor.
/// These futures must return `()`. The futures added here are intended to driver side-effects
/// only. Use `add_future` for top-level futures.
pub fn add_future(future: Pin<Box<dyn Future<Output = ()>>>) {
    FUTURES.with(|new_futures| {
        let mut new_futures = new_futures.borrow_mut();
        new_futures.push((future, AtomicBool::new(true)));
    });
}

/// Runs futures to completion on a single thread. Futures are allowed to block on file descriptors
/// only. Futures can only block on FDs becoming readable or writable. `FdExecutor` is meant to be
/// used where a poll or select loop would be used otherwise.
pub struct FdExecutor {
    futures: Vec<(Pin<Box<dyn Future<Output = ()>>>, AtomicBool)>,
    poll_ctx: PollContext<u64>,
    token_map: BTreeMap<u64, (SavedFd, Waker)>,
    next_token: u64, // Next token for adding to the poll context.
}

impl FdExecutor {
    /// Create a new executor.
    pub fn new() -> FdExecutor {
        FdExecutor {
            futures: Vec::new(),
            poll_ctx: PollContext::new().unwrap(),
            token_map: BTreeMap::new(),
            next_token: 0,
        }
    }

    /// Appends the given future to the list of futures to run.
    /// These futures must return `()`. The futures added here are intended to drive side-effects
    /// only. Use `add_future` for top-level futures.
    pub fn add_future(&mut self, future: Pin<Box<dyn Future<Output = ()>>>) {
        self.futures.push((future, AtomicBool::new(true)));
    }

    /// Runs the given future until it completes. The future must return () as the `Output`.
    pub fn run_one(future: Pin<Box<dyn Future<Output = ()>>>) {
        let mut ex = Self::new();
        ex.add_future(future);
        ex.run()
    }

    /// Run the executor until any future completes, returns once any of the futures added to it
    /// have completed.
    pub fn run_first(&mut self) {
        self.run_all(true)
    }

    /// Run the executor, this will return once all of the futures added to it have completed.
    pub fn run(&mut self) {
        self.run_all(false)
    }

    /// Run the executor, this will consume the executor and return once any of the futures
    /// added to it have completed. If 'exit_any' is true, 'run_all' returns after any future
    /// completes. If 'exit_any' is false, only return after all futures have completed.
    fn run_all(&mut self, exit_any: bool) {
        loop {
            // for each future that is ready:
            //  poll it
            //  remove it if ready
            let mut i = 0;
            while i < self.futures.len() {
                // The loop would be `drain_filter` if it was stable.
                let (fut, ready) = &mut self.futures[i];

                if ready.swap(false, Ordering::Relaxed) && poll_one(fut, ready) == Poll::Ready(()) {
                    self.futures.remove(i);
                    if exit_any {
                        return;
                    }
                } else {
                    i += 1;
                }
            }

            // Add any new futures and wakers to the lists.
            FUTURES.with(|new_futures| {
                let mut new_futures = new_futures.borrow_mut();
                self.futures.append(&mut new_futures);
            });

            FDS.with(|new_fds| {
                let mut new_fds = new_fds.borrow_mut();
                for (saved_fd, waker, events) in new_fds.drain(..) {
                    self.add_waker(saved_fd, waker, events);
                }
            });

            if self.futures.is_empty() {
                return;
            }

            // If no futures are read, sleep until a waker is signaled.
            if !self
                .futures
                .iter()
                .any(|(_fut, ready)| ready.load(Ordering::Relaxed))
            {
                self.wait_wake_event();
            }
        }

        fn poll_one(
            future: &mut Pin<Box<dyn Future<Output = ()>>>,
            ready: &mut AtomicBool,
        ) -> Poll<()> {
            // Safe because a valid pointer is passed to `create_waker` and the valid result is
            // passed to `Waker::from_raw`.
            let waker = unsafe {
                let raw_waker = create_waker(ready as *const _ as *const _);
                Waker::from_raw(raw_waker)
            };
            let mut ctx = Context::from_waker(&waker);
            let f = future.as_mut();
            f.poll(&mut ctx)
        }
    }

    // Adds an fd that, when signaled, will trigger the given waker.
    fn add_waker(&mut self, fd: SavedFd, waker: Waker, events: WatchingEvents) {
        while self.token_map.contains_key(&self.next_token) {
            self.next_token += 1;
        }
        self.poll_ctx
            .add_fd_with_events(&fd, events, self.next_token)
            .unwrap();
        let next_token = self.next_token;
        self.token_map.insert(next_token, (fd, waker));
    }

    /// Waits until one of the FDs is readable and wakes the associated waker.
    pub fn wait_wake_event(&mut self) {
        let events = self.poll_ctx.wait().unwrap();
        for e in events.iter() {
            if let Some((fd, waker)) = self.token_map.remove(&e.token()) {
                self.poll_ctx.delete(&fd).unwrap();
                waker.wake_by_ref();
            }
        }
    }
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

#[cfg(test)]
mod tests {
    use crate::{add_read_waker, add_write_waker, FdExecutor};
    use futures::io::{AsyncRead, AsyncWrite};
    use futures::io::{AsyncReadExt, AsyncWriteExt};
    use std::fs::File;
    use std::io::{self, ErrorKind, Read, Write};
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use sys_util::pipe_non_blocking;

    struct AsyncRx(File);
    struct AsyncTx(File);

    fn async_pipes() -> (AsyncRx, AsyncTx) {
        let (rx, tx) = pipe_non_blocking(true).unwrap();
        (AsyncRx(rx), AsyncTx(tx))
    }
    impl AsyncRead for AsyncRx {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context,
            buf: &mut [u8],
        ) -> Poll<Result<usize, io::Error>> {
            match self.0.read(buf) {
                Ok(len) => Poll::Ready(Ok(len)),
                Err(e) => {
                    if e.kind() == ErrorKind::WouldBlock {
                        add_read_waker(&self.0, cx.waker().clone());
                        Poll::Pending
                    } else {
                        Poll::Ready(Err(e))
                    }
                }
            }
        }
    }
    impl AsyncWrite for AsyncTx {
        fn poll_write(
            mut self: Pin<&mut Self>,
            cx: &mut Context,
            buf: &[u8],
        ) -> Poll<Result<usize, io::Error>> {
            match self.0.write(buf) {
                Ok(len) => Poll::Ready(Ok(len)),
                Err(e) => {
                    if e.kind() == ErrorKind::WouldBlock {
                        add_write_waker(&self.0, cx.waker().clone());
                        Poll::Pending
                    } else {
                        Poll::Ready(Err(e))
                    }
                }
            }
        }

        fn poll_flush(mut self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Result<(), io::Error>> {
            &self.0.flush().unwrap(); // This could block but it doesn't matter for the tests.
            Poll::Ready(Ok(()))
        }

        fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), io::Error>> {
            self.poll_flush(cx)
        }
    }

    // A read and write closure are created and then added to the executor.
    // They take turns blocking on the other sending a message.
    #[test]
    fn communicate_cross_closure() {
        let (data_rx, data_tx) = async_pipes();
        let (ack_rx, ack_tx) = async_pipes();

        async fn handle_read(mut data_rx: AsyncRx, mut ack_tx: AsyncTx) {
            let mut buf = [0x55u8; 48];
            data_rx.read_exact(&mut buf).await.expect("Failed to read");
            assert!(buf.iter().all(|&e| e == 0x00));
            ack_tx.write_all(&[b'a']).await.unwrap();
            data_rx.read_exact(&mut buf).await.expect("Failed to read");
            assert!(buf.iter().all(|&e| e == 0xaa));
        }

        async fn handle_write(mut data_tx: AsyncTx, mut ack_rx: AsyncRx) {
            let zeros = [0u8; 48];
            data_tx.write_all(&zeros).await.unwrap();
            let mut ack = [0u8];
            assert!(ack_rx.read_exact(&mut ack).await.is_ok());
            let aas = [0xaau8; 48];
            data_tx.write_all(&aas).await.unwrap();
        }

        let mut ex = FdExecutor::new();
        ex.add_future(Box::pin(handle_read(data_rx, ack_tx)));
        ex.add_future(Box::pin(handle_write(data_tx, ack_rx)));

        ex.run();
    }
}
