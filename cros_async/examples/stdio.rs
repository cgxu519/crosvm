// Copyright 2019 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::future::Future;
use std::io::{stdin, Read, StdinLock};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::os::unix::io::AsRawFd;

use cros_async::{empty_executor, Executor};
use cros_async::fd_executor::{add_future, add_read_waker};

struct VectorProducer<'a> {
    stdin_lock: StdinLock<'a>,
    started: bool, // hack because first poll can't check stdin for readable.
}

impl<'a> VectorProducer<'a> {
    pub fn new(stdin_lock: StdinLock<'a>) -> Self {
        VectorProducer {
            stdin_lock,
            started: false,
        }
    }
}

impl<'a> Future for VectorProducer<'a> {
    type Output = Vec<u8>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        println!("poll");
        if self.started {
            let mut b = [0u8; 2];
            self.stdin_lock.read(&mut b).unwrap();
            if b[0] >= b'0' && b[0] <= b'9' {
                return Poll::Ready((0..(b[0] - b'0')).collect());
            }
        }
        self.started = true;
        let _ = add_read_waker(self.stdin_lock.as_raw_fd(), cx.waker().clone());
        Poll::Pending
    }
}

fn main() {
    let ex = empty_executor();

    async fn get_vec() {
        let stdin = stdin();
        let stdin_lock = stdin.lock();

        let vec_future = VectorProducer::new(stdin_lock);
        println!("async closure.");
        let buf = vec_future.await;
        println!("async closure after await {}.", buf.len());
    }
    println!("pre add future");

    add_future(Box::pin(get_vec()));
    println!("after adding, before running");

    let _ = ex.map(|mut f| f.run());
}
