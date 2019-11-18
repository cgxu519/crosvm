// Copyright 2019 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::future::Future;
use std::io;
use std::io::Read;
use std::os::unix::io::AsRawFd;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::add_read_waker;

pub trait ReadAsync {
    type Reader: Read + AsRawFd;

    fn read_all_async<'a, 'b>(&'a mut self, buf: &'b mut [u8]) -> AwaitRead<'a, 'b, Self::Reader>;
}

pub struct AwaitRead<'a, 'b, T>
where
    T: Read + AsRawFd,
{
    reader: &'a mut T,
    buf: &'b mut [u8],
    len_read: Option<usize>,
}

impl<'a, 'b, T> Future for AwaitRead<'a, 'b, T>
where
    T: Read + AsRawFd,
{
    type Output = io::Result<usize>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // Safe because nothing is moved out of the self reference.
        unsafe {
            // unsafe as long as we hold an raw mut ref to the Pinned self.
            let s = self.as_mut().get_unchecked_mut();
            if let Some(ref mut len_read) = s.len_read {
                // Started reading, assume that there is data.
                match s.reader.read(&mut s.buf[*len_read..]) {
                    Ok(n) => *len_read += n,
                    Err(e) => return Poll::Ready(Err(e)),
                }
                if *len_read >= s.buf.len() {
                    return Poll::Ready(Ok(*len_read));
                }
            } else {
                // TODO - should there be a non-blocking read before the first wakeup is scheduled
                s.len_read = Some(0);
            }
            add_read_waker(s.reader, cx.waker().clone());
        }

        Poll::Pending
    }
}

impl<T: Read + AsRawFd> ReadAsync for T {
    type Reader = T;

    fn read_all_async<'a, 'b>(&'a mut self, buf: &'b mut [u8]) -> AwaitRead<'a, 'b, Self::Reader> {
        AwaitRead {
            buf,
            reader: self,
            len_read: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ReadAsync;
    use crate::FdExecutor;
    use std::io::Write;
    use sys_util::pipe;

    // A read and write cloure are created and then added to the executor.
    // They take turns blocking on the other sending a message.
    #[test]
    fn communicate_cross_closure() {
        let (mut data_rx, mut data_tx) = pipe(true).unwrap();
        let (mut ack_rx, mut ack_tx) = pipe(true).unwrap();

        let read_closure = move || {
            async move {
                let mut buf = [0x55u8; 48];
                data_rx
                    .read_all_async(&mut buf)
                    .await
                    .expect("Failed to read");
                assert!(buf.iter().all(|&e| e == 0x00));
                ack_tx.write(&[b'a']).unwrap();
                data_rx
                    .read_all_async(&mut buf)
                    .await
                    .expect("Failed to read");
                assert!(buf.iter().all(|&e| e == 0xaa));
            }
        };
        let read_future = read_closure();

        let write_closure = move || {
            async move {
                let zeros = [0u8; 48];
                data_tx.write(&zeros).unwrap();
                let mut ack = [0u8];
                assert_eq!(ack_rx.read_all_async(&mut ack).await.unwrap(), 1usize);
                let aas = [0xaau8; 48];
                data_tx.write(&aas).unwrap();
            }
        };
        let write_future = write_closure();

        let mut ex = FdExecutor::new();
        ex.add_future(Box::pin(read_future));
        ex.add_future(Box::pin(write_future));

        ex.run();
    }
}
