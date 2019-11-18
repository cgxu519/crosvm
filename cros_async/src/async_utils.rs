// Copyright 2019 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Extentions to sys-util adding asynchronous operations.
use futures::Stream;
use std::convert::TryFrom;
use std::mem;
use std::os::unix::io::{AsRawFd, RawFd};
use std::pin::Pin;
use std::task::{Context, Poll};

use libc::{c_int, c_void, fcntl, read, EWOULDBLOCK, F_GETFL, F_SETFL, O_NONBLOCK};

use msg_socket::{MsgError, MsgOnSocket, MsgReceiver, MsgSocket};
use sys_util::{self, Error, EventFd, Result};

use crate::add_read_waker;

pub struct AsyncEventFd(EventFd);

impl AsyncEventFd {
    pub fn new() -> Result<AsyncEventFd> {
        Self::try_from(EventFd::new()?)
    }
}

impl TryFrom<EventFd> for AsyncEventFd {
    type Error = sys_util::Error;

    fn try_from(eventfd: EventFd) -> Result<AsyncEventFd> {
        let fd = eventfd.as_raw_fd();
        let flags = get_flags(fd)?;
        set_flags(fd, flags | O_NONBLOCK)?;
        Ok(AsyncEventFd(eventfd))
    }
}

impl Stream for AsyncEventFd {
    type Item = u64;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let mut buf: u64 = 0;
        let ret = unsafe {
            // This is safe because we made this fd and the pointer we pass can not overflow because
            // we give the syscall's size parameter properly.
            read(
                self.0.as_raw_fd(),
                &mut buf as *mut u64 as *mut c_void,
                mem::size_of::<u64>(),
            )
        };
        if ret <= 0 {
            let err = Error::last();
            if err.errno() == EWOULDBLOCK {
                add_read_waker(&self.0, cx.waker().clone());
                return Poll::Pending;
            } else {
                // Indicate something went wrong and no more events will be provided.
                return Poll::Ready(None);
            }
        }
        Poll::Ready(Some(buf))
    }
}

fn get_flags(fd: RawFd) -> Result<c_int> {
    // Safe because no third parameter is expected and we check the return result.
    let ret = unsafe { fcntl(fd, F_GETFL) };
    if ret < 0 {
        return Err(Error::last());
    }
    Ok(ret)
}

fn set_flags(fd: RawFd, flags: c_int) -> Result<()> {
    // Safe because we supply the third parameter and we check the return result.
    let ret = unsafe { fcntl(fd, F_SETFL, flags) };
    if ret < 0 {
        return Err(Error::last());
    }
    Ok(())
}

// TODO(dgreid)Maybe Receiver instead???
pub struct AsyncReceiver<I: MsgOnSocket, O: MsgOnSocket>(MsgSocket<I, O>);

impl<I: MsgOnSocket, O: MsgOnSocket> TryFrom<MsgSocket<I, O>> for AsyncReceiver<I, O> {
    type Error = sys_util::Error;

    fn try_from(sock: MsgSocket<I, O>) -> Result<AsyncReceiver<I, O>> {
        let fd = sock.as_raw_fd();
        let flags = get_flags(fd)?;
        set_flags(fd, flags | O_NONBLOCK)?;
        Ok(AsyncReceiver(sock))
    }
}

impl<I: MsgOnSocket, O: MsgOnSocket> Stream for AsyncReceiver<I, O> {
    type Item = O;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match self.0.recv() {
            Ok(msg) => Poll::Ready(Some(msg)),
            Err(MsgError::Recv(e)) => {
                if e.errno() == EWOULDBLOCK {
                    add_read_waker(&self.0, cx.waker().clone());
                    Poll::Pending
                } else {
                    // Indicate something went wrong and no more events will be provided.
                    Poll::Ready(None)
                }
            }
            Err(_) => {
                // Indicate something went wrong and no more events will be provided.
                Poll::Ready(None)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FdExecutor;
    use futures::stream::StreamExt;

    #[test]
    fn eventfd_write_read() {
        let evt = AsyncEventFd::new().unwrap();
        evt.0.write(55).unwrap();
        async fn read_one(mut evt: AsyncEventFd) {
            if let Some(e) = evt.next().await {
                assert_eq!(e, 55);
            }
        }
        let mut ex = FdExecutor::new();
        ex.add_future(Box::pin(read_one(evt)));
        ex.run();
    }
}
