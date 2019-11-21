// Copyright 2019 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! An Executor and base futures based on operation that block on system file desrciptors.
//!
//! There are basic Futures provided for some of the interfaces provided byt the `sys_util` crate.
//! These live under `async_utils`.
//!
//! This crate is meant to be used with the `futures-rs` crate that provides combinators and
//! utility functions to combine futures.
//!
//! The `FdExecutor` runs provided futures to completion, using FDs to wake the individual tasks.
//!
//! # Implementing new FD-based futures.
//!
//! When building futures to be run in an `FdExecutor` framework, use the following helper functions
//! to perform common tasks:
//!
//! `add_read_waker` - Used to associate a provided FD becoming readable with the future being
//! woken.
//!
//! `add_write_waker` - Used to associate a provided FD becoming writable with the future being
//! woken.
//!
//! `add_future` - Used to add a new future to the top-level list of running futures.
//!
//! # Example with asynchronous pipes
//!
//! ```
//! use cros_async::{add_read_waker, add_write_waker, FdExecutor};
//! use futures::io::{AsyncRead, AsyncWrite};
//! use futures::io::{AsyncReadExt, AsyncWriteExt};
//! use std::fs::File;
//! use std::io::{self, ErrorKind, Read, Write};
//! use std::pin::Pin;
//! use std::task::{Context, Poll};
//! use sys_util::pipe_non_blocking;
//!
//! struct AsyncRx(File);
//! struct AsyncTx(File);
//!
//!
//! impl AsyncRead for AsyncRx {
//!     fn poll_read(
//!         mut self: Pin<&mut Self>,
//!         cx: &mut Context,
//!         buf: &mut [u8],
//!     ) -> Poll<Result<usize, io::Error>> {
//!         match self.0.read(buf) {
//!             Ok(len) => Poll::Ready(Ok(len)),
//!             Err(e) => {
//!                 if e.kind() == ErrorKind::WouldBlock {
//!                     // The FD would block, tell the executor to wake the future when the backing
//!                     // FD is readable
//!                     add_read_waker(&self.0, cx.waker().clone());
//!                     Poll::Pending
//!                 } else {
//!                     Poll::Ready(Err(e))
//!                 }
//!             }
//!         }
//!     }
//! }
//!
//! impl AsyncWrite for AsyncTx {
//!     fn poll_write(
//!         mut self: Pin<&mut Self>,
//!         cx: &mut Context,
//!         buf: &[u8],
//!     ) -> Poll<Result<usize, io::Error>> {
//!         match self.0.write(buf) {
//!             Ok(len) => Poll::Ready(Ok(len)),
//!             Err(e) => {
//!                 if e.kind() == ErrorKind::WouldBlock {
//!                     // The FD would block, tell the executor to wake the future when the backing
//!                     // FD is writable.
//!                     add_write_waker(&self.0, cx.waker().clone());
//!                     Poll::Pending
//!                 } else {
//!                     Poll::Ready(Err(e))
//!                 }
//!             }
//!         }
//!     }
//!
//!     fn poll_flush(
//!         mut self: Pin<&mut Self>,
//!         _cx: &mut Context
//!     ) -> Poll<Result<(), io::Error>> {
//!         &self.0.flush().unwrap(); // This could block but it doesn't matter for the example.
//!         Poll::Ready(Ok(()))
//!     }
//!
//!     fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), io::Error>> {
//!         self.poll_flush(cx)
//!     }
//! }
//!
//! // A read and write closure are created and then added to the executor.
//! // They take turns blocking on the other sending a message.
//! fn communicate_cross_closure() {
//!     fn async_pipes() -> (AsyncRx, AsyncTx) {
//!         let (rx, tx) = pipe_non_blocking(true).unwrap();
//!         (AsyncRx(rx), AsyncTx(tx))
//!     }
//!
//!     let (data_rx, data_tx) = async_pipes();
//!     let (ack_rx, ack_tx) = async_pipes();
//!
//!     async fn handle_read(mut data_rx: AsyncRx, mut ack_tx: AsyncTx) {
//!         let mut buf = [0x55u8; 48];
//!         data_rx.read_exact(&mut buf).await.expect("Failed to read");
//!         assert!(buf.iter().all(|&e| e == 0x00));
//!         ack_tx.write_all(&[b'a']).await.unwrap();
//!         data_rx.read_exact(&mut buf).await.expect("Failed to read");
//!         assert!(buf.iter().all(|&e| e == 0xaa));
//!     }
//!
//!     async fn handle_write(mut data_tx: AsyncTx, mut ack_rx: AsyncRx) {
//!         let zeros = [0u8; 48];
//!         data_tx.write_all(&zeros).await.unwrap();
//!         let mut ack = [0u8];
//!         assert!(ack_rx.read_exact(&mut ack).await.is_ok());
//!         let aas = [0xaau8; 48];
//!         data_tx.write_all(&aas).await.unwrap();
//!     }
//!
//!     let mut ex = FdExecutor::new();
//!     ex.add_future(Box::pin(handle_read(data_rx, ack_tx)));
//!     ex.add_future(Box::pin(handle_write(data_tx, ack_rx)));
//!
//!     ex.run();
//! }
//! ```

mod aio;
mod aio_abi_bindings;
mod async_utils;
mod fd_executor;

pub use async_utils::{AsyncEventFd, AsyncReceiver};
pub use fd_executor::{add_future, add_read_waker, add_write_waker, FdExecutor};
