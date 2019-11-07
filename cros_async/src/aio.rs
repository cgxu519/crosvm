// Copyright 2019 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use crate::aio_abi_bindings::{
    aio_context_t, io_event, io_getevents, io_setup, io_submit, iocb, IOCB_CMD_PREAD,
};

use std::future::Future;
use std::iter::{FromIterator, IntoIterator};
use std::mem;
use std::os::unix::io::AsRawFd;
use std::pin::Pin;
use std::result::Result;
use std::task::{Context, Poll};

pub struct AioJob {
    context: aio_context_t,
    result: Option<Result<usize, ()>>, //TODO should be a list of transaction sizes not a usize.
    to_complete: usize,
}

// TODO: also impl for volatile slice.
// TODO: Add an option to createing the iocbs and then passing htem to an existing context. This
// will save on io_setup calls.
impl<'a, T> FromIterator<(T, &'a mut [u8])> for AioJob
where
    T: AsRawFd,
{
    fn from_iter<I: IntoIterator<Item = (T, &'a mut [u8])>>(slices: I) -> Self {
        let context = match io_setup(128) {
            Ok(c) => c,
            Err(_) => {
                return AioJob {
                    context: Default::default(),
                    result: Some(Err(())),
                    to_complete: 0,
                };
            }
        };
        // TODO: add iocb struct per slice
        let mut num_cb = 0;
        let mut iocbs = [iocb::default(); 128];
        for ((fd, slice), cb) in slices.into_iter().zip(iocbs.iter_mut()) {
            cb.aio_fildes = fd.as_raw_fd() as u32;
            cb.aio_buf = unsafe {
                // Transmute is safe because the raw pointer will be the same size as the aio_buf
                // field by the definition in the kernel ABI.
                mem::transmute(slice.as_mut_ptr())
            };
            cb.aio_lio_opcode = IOCB_CMD_PREAD as u16;
            cb.aio_nbytes = slice.len() as u64;

            num_cb += 1;
        }

        unsafe {
            // Safe because each entry in callbacks is configred above to write to the volatile
            // slice this function has a refernce to.
            match io_submit(&iocbs[0..num_cb]) {
                Ok(()) => (),
                Err(_) => {
                    return AioJob {
                        context: Default::default(),
                        result: Some(Err(())),
                        to_complete: 0,
                    };
                }
            }
        }

        AioJob {
            context,
            result: None,
            to_complete: num_cb,
        }
    }
}

impl Future for AioJob {
    type Output = Result<usize, ()>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        if let Some(r) = self.result.take() {
            return Poll::Ready(r);
        }

        // TODO MaybeUninit
        let mut events = [io_event::default(); 128];
        let ret = match io_getevents(self.context, &mut events[0..self.to_complete]) {
            Err(_) => return Poll::Ready(Err(())),
            Ok(r) => r,
        };
        self.to_complete -= ret;
        if self.to_complete == 0 {
            return Poll::Ready(Ok(0));
        }
        // TODO - register with the executor to be woken when the fd is ready.
        Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn collect_to_future() {
        // let file = Open...;
        // Vector of mutable [volatile]slices
        // collect iterator to an AioJob
        // aio_job.await
    }

    #[test]
    fn write_one() {
        // let file = Open...;
        //let mut aio_context = AioContext::new(&file); // new takes Deref to AsRawFd
        // create volatile slice
        // aio_context.add_volatile_slice(vs, read, offset);
        // submit context
        // poll on events
        // read events
    }
}
