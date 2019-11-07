// Copyright 2019 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(PartialEq)]
enum FutureState<F: Future> {
    Ready(F),
    Finished(F::Output),
    Consumed,
}

impl<F: Future> FutureState<F> {
    pub fn new(fut: F) -> FutureState<F> {
        FutureState::Ready(fut)
    }

    /// Takes the output from a completed future.
    pub fn output(self: Pin<&mut Self>) -> Option<F::Output> {
        //Safe to modify self because this is a consuming operation and self can't be reused.
        // TODO - is it really safe?
        let s = unsafe { self.get_unchecked_mut() };
        match s {
            FutureState::Ready(_) => return None,
            FutureState::Finished(_) => (),
            FutureState::Consumed => return None,
        }
        if let FutureState::Finished(v) = std::mem::replace(s, FutureState::Consumed) {
            return Some(v);
        }
        unreachable!()
    }
}

impl<F: Future> Future for FutureState<F> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // safe because nothing is moved out of the reference.
        let ret = unsafe {
            match self.as_mut().get_unchecked_mut() {
                // TODO is new_unchecked actually safe here?
                FutureState::Ready(f) => Pin::new_unchecked(f).poll(cx),
                FutureState::Finished(_) => return Poll::Ready(()),
                FutureState::Consumed => panic!("poll result already consumed"),
            }
        };
        if let Poll::Ready(v) = ret {
            self.set(FutureState::Finished(v));
            return Poll::Ready(());
        }
        return Poll::Pending;
    }
}

// Returns a single future that returns a tuple of the outputs of the two specified futures.
pub fn await_two<F1: Future, F2: Future>(f1: F1, f2: F2) -> AwaitTwo<F1, F2> {
    AwaitTwo {
        first: FutureState::new(f1),
        second: FutureState::new(f2),
    }
}

pub struct AwaitTwo<F, G>
where
    F: Future,
    G: Future,
{
    first: FutureState<F>,
    second: FutureState<G>,
}

impl<F, G> Future for AwaitTwo<F, G>
where
    F: Future,
    G: Future,
{
    type Output = (F::Output, G::Output);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        unsafe {
            // TODO - check that Pin::map_unchecked_mut is safe here
            if let (Poll::Ready(()), Poll::Ready(())) = (
                Pin::map_unchecked_mut(self.as_mut(), |f| &mut f.first).poll(cx),
                Pin::map_unchecked_mut(self.as_mut(), |f| &mut f.second).poll(cx),
            ) {
                Poll::Ready((
                    Pin::map_unchecked_mut(self.as_mut(), |f| &mut f.first)
                        .output()
                        .unwrap(),
                    Pin::map_unchecked_mut(self.as_mut(), |f| &mut f.second)
                        .output()
                        .unwrap(),
                ))
            } else {
                Poll::Pending
            }
        }
    }
}
