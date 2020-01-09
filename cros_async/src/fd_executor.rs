// Copyright 2020 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! The executor runs all given futures to completion. Futures register wakers associated with file
//! descriptors. The wakers will be called when the FD becomes readable or writable depending on
//! the situation.
//!
//! `FdExecutor` is meant to be used with the `futures-rs` crate that provides combinators and
//! utility functions to combine futures.
//!
//! # Example of starting the framework and running a future:
//!
//! ```
//! use cros_async::Executor;
//! async fn my_async() {
//!     // Insert async code here.
//! }
//!
//! let mut ex = cros_async::empty_executor().expect("Failed creating executor");
//! cros_async::fd_executor::add_future(Box::pin(my_async()));
//! ex.run();
//! ```

use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::fmt::{self, Display};
use std::future::Future;
use std::os::unix::io::{AsRawFd, RawFd};
use std::pin::Pin;
use std::task::Waker;

use sys_util::{PollContext, WatchingEvents};

use crate::executor::{ExecutableFuture, Executor, FutureList};

#[derive(Debug, PartialEq)]
pub enum Error {
    /// Failed to copy the FD for the polling context.
    DuplicatingFd(sys_util::Error),
    /// Failed accessing the thread local storage for wakers.
    InvalidContext,
    /// Creating a context to wait on FDs failed.
    CreatingContext(sys_util::Error),
    /// Failed to submit the waker to the polling context.
    SubmittingWaker(sys_util::Error),
}
pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match self {
            DuplicatingFd(e) => write!(f, "Failed to copy the FD for the polling context: {}", e),
            InvalidContext => write!(
                f,
                "Invalid context, was the Fd executor created successfully?"
            ),
            CreatingContext(e) => write!(f, "An Error creating the fd waiting context: {}.", e),
            SubmittingWaker(e) => write!(f, "An Error adding to the Aio context: {}.", e),
        }
    }
}

// Temporary vectors of new additions to the executor.

// Top level futures that are added during poll calls.
thread_local!(static NEW_FUTURES: RefCell<VecDeque<ExecutableFuture<()>>> =
              RefCell::new(VecDeque::new()));

// Tracks active wakers and the futures they are associated with.
thread_local!(static STATE: RefCell<Option<FdWakerState>> = RefCell::new(None));

fn add_waker(fd: RawFd, waker: Waker, events: WatchingEvents) -> Result<()> {
    STATE.with(|waker_state| {
        let mut waker_state = waker_state.borrow_mut();
        if let Some(waker_state) = waker_state.as_mut() {
            waker_state.add_waker(fd, waker, events)
        } else {
            Err(Error::InvalidContext)
        }
    })
}

/// Tells the waking system to wake `waker` when `fd` becomes readable.
/// The 'fd' must be fully owned by the future adding the waker, and must not be closed until the
/// next time the future is polled. If the fd is closed, there is a race where another FD can be
/// opened on top of it causing the next poll to access the new target file.
pub fn add_read_waker(fd: RawFd, waker: Waker) -> Result<()> {
    add_waker(fd, waker, WatchingEvents::empty().set_read())
}

/// Tells the waking system to wake `waker` when `fd` becomes writable.
/// The 'fd' must be fully owned by the future adding the waker, and must not be closed until the
/// next time the future is polled. If the fd is closed, there is a race where another FD can be
/// opened on top of it causing the next poll to access the new target file.
pub fn add_write_waker(fd: RawFd, waker: Waker) -> Result<()> {
    add_waker(fd, waker, WatchingEvents::empty().set_write())
}

/// Adds a new top level future to the Executor.
/// These futures must return `()`, indicating they are intended to create side-effects only.
pub fn add_future(future: Pin<Box<dyn Future<Output = ()>>>) {
    NEW_FUTURES.with(|new_futures| {
        let mut new_futures = new_futures.borrow_mut();
        new_futures.push_back(ExecutableFuture::new(future));
    });
}

// Tracks active wakers and associates wakers with the futures that registered them.
struct FdWakerState {
    poll_ctx: PollContext<u64>,
    token_map: BTreeMap<u64, (DupedFd, Waker)>,
    next_token: u64, // Next token for adding to the context.
}

impl FdWakerState {
    fn new() -> Result<Self> {
        Ok(FdWakerState {
            poll_ctx: PollContext::new().map_err(Error::CreatingContext)?,
            token_map: BTreeMap::new(),
            next_token: 0,
        })
    }

    // Adds an fd that, when signaled, will trigger the given waker.
    fn add_waker(&mut self, fd: RawFd, waker: Waker, events: WatchingEvents) -> Result<()> {
        let duped_fd = DupedFd::new(fd)?;
        while self.token_map.contains_key(&self.next_token) {
            self.next_token += 1;
        }
        self.poll_ctx
            .add_fd_with_events(&duped_fd, events, self.next_token)
            .map_err(Error::SubmittingWaker)?;
        let next_token = self.next_token;
        self.token_map.insert(next_token, (duped_fd, waker));
        Ok(())
    }

    // Waits until one of the FDs is readable and wakes the associated waker.
    fn wait_wake_event(&mut self) {
        let events = self.poll_ctx.wait().unwrap(); //TODO unwrap.
        for e in events.iter() {
            if let Some((fd, waker)) = self.token_map.remove(&e.token()) {
                self.poll_ctx.delete(&fd).unwrap(); // TODO unwrap.
                waker.wake_by_ref();
            }
        }
    }
}

/// Runs futures to completion on a single thread. Futures are allowed to block on file descriptors
/// only. Futures can only block on FDs becoming readable or writable. `FdExecutor` is meant to be
/// used where a poll or select loop would be used otherwise.
pub(crate) struct FdExecutor<T: FutureList> {
    futures: T,
}

impl<T: FutureList> Executor for FdExecutor<T> {
    type Output = T::Output;

    fn run(&mut self) -> Self::Output {
        loop {
            if let Some(output) = self.futures.poll_results() {
                return output;
            }

            // Add any new futures and wakers to the lists.
            NEW_FUTURES.with(|new_futures| {
                let mut new_futures = new_futures.borrow_mut();
                self.futures.futures_mut().append(&mut new_futures);
            });

            // If no futures are ready, sleep until a waker is signaled.
            if !self.futures.any_ready() {
                STATE.with(|waker_state| {
                    let mut waker_state = waker_state.borrow_mut();
                    if let Some(waker_state) = waker_state.as_mut() {
                        waker_state.wait_wake_event();
                    } else {
                        unreachable!("Can't get here without a context being created");
                    }
                });
            }
        }
    }
}

impl<T: FutureList> FdExecutor<T> {
    /// Create a new executor.
    pub fn new(futures: T) -> Result<FdExecutor<T>> {
        STATE.with(|waker_state| {
            waker_state.replace(Some(FdWakerState::new()?));
            Ok(())
        })?;
        Ok(FdExecutor { futures })
    }
}

impl<T: FutureList> Drop for FdExecutor<T> {
    fn drop(&mut self) {
        STATE.with(|waker_state| {
            waker_state.replace(None);
        });
    }
}

// Track fd's after they are duped until they are added to the poll context.
// Once the fd is in the poll context, it can be closed, which is handled by drop here.
struct DupedFd(RawFd);

impl DupedFd {
    fn new(fd: RawFd) -> Result<DupedFd> {
        Ok(DupedFd(unsafe {
            // Safe because duplicating an FD doesn't affect memory safety, and the dup'd FD
            // will only be added to the poll loop.
            Self::dup_fd(fd)?
        }))
    }

    // Used to dup the FDs passed to the executor so there is a guarantee they aren't closed while
    // waiting in TLS to be added to the main polling context.
    unsafe fn dup_fd(fd: RawFd) -> Result<RawFd> {
        let ret = libc::dup(fd);
        if ret < 0 {
            Err(Error::DuplicatingFd(sys_util::Error::last()))
        } else {
            Ok(ret)
        }
    }
}

impl Drop for DupedFd {
    fn drop(&mut self) {
        // Safe to close the dup'd FD here as the kernel has another ref to it once it
        // has been added to the context in `add_waker`.
        unsafe {
            libc::close(self.0);
        }
    }
}

impl AsRawFd for DupedFd {
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}