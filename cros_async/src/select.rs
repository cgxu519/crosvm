// Copyright 2020 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Need non-snake case so the macro can re-use type names for variables.
#![allow(non_snake_case)]

use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Context;
use std::task::Waker;

use futures::future::{maybe_done, MaybeDone};

use crate::executor::{FutureList, UnitFutures};
use crate::waker::create_waker;

pub enum SelectResult<F: Future> {
    Pending(F),
    Finished(F::Output),
}

// Macro-generate future combinators to allow for running different numbers of top-level futures in
// this FutureList. Generates the implementation of `FutureList` for the select types. For an
// explicit example this is modeled after, see `UnitFutures`.
macro_rules! generate {
    ($(
        $(#[$doc:meta])*
        ($Select:ident, <$($Fut:ident),*>),
    )*) => ($(
        $(#[$doc])*
        #[must_use = "Combinations of futures don't do anything unless run in an executor."]
        paste::item! {
            pub(crate) struct $Select<$($Fut: Future),*> {
                added_futures: UnitFutures,
                $($Fut: MaybeDone<$Fut>,)*
                $([<$Fut _ready>]: Rc<AtomicBool>,)*
            }
        }

        impl<$($Fut: Future),*> $Select<$($Fut),*> {
            paste::item! {
                pub(crate) fn new($($Fut: $Fut),*) -> $Select<$($Fut),*> {
                    $Select {
                        added_futures: UnitFutures::new(),
                        $($Fut: maybe_done($Fut),)*
                        $([<$Fut _ready>]: Rc::new(AtomicBool::new(true)),)*
                    }
                }
            }
        }

        impl<$($Fut: Future),*> FutureList for $Select<$($Fut),*> {
            type Output = ($(SelectResult<$Fut>),*);

            fn futures_mut(&mut self) -> &mut UnitFutures {
                &mut self.added_futures
            }

            paste::item! {
                fn poll_results(&mut self) -> Option<Self::Output> {
                    let _ = self.added_futures.poll_results();

                    let mut complete = false;
                    $(
                        let $Fut = unsafe {
                            // Safe because no future will be moved before the structure is dropped
                            // and no future can run after the structure is dropped.
                            Pin::new_unchecked(&mut self.$Fut)
                        };
                        if self.[<$Fut _ready>].swap(false, Ordering::Relaxed) {
                            let waker = unsafe {
                                let clone = self.[<$Fut _ready>].clone();
                                let raw_waker = create_waker(Rc::into_raw(clone) as *const _);
                                Waker::from_raw(raw_waker)
                            };
                            let mut ctx = Context::from_waker(&waker);
                            complete |= $Fut.poll(&mut ctx).is_ready();
                        }
                    )*

                    if complete {
                        Some(($(
                                    match std::mem::replace(&mut self.$Fut, MaybeDone::Gone) {
                                        MaybeDone::Future(f) => SelectResult::Pending(f),
                                        MaybeDone::Done(o) => SelectResult::Finished(o),
                                        MaybeDone::Gone=>unreachable!(),
                                    }
                               ), *))
                    } else {
                        None
                    }
                }

                fn any_ready(&self) -> bool {
                    let mut ready = self.added_futures.any_ready();
                    $(
                        ready |= self.[<$Fut _ready>].load(Ordering::Relaxed);
                    )*
                    ready
                }
            }
        }
    )*)
}

generate! {
    /// Future for the [`select2`] function.
    (Select2, <Fut1, Fut2>),

    /// Future for the [`select3`] function.
    (Select3, <Fut1, Fut2, Fut3>),

    /// Future for the [`select4`] function.
    (Select4, <Fut1, Fut2, Fut3, Fut4>),

    /// Future for the [`select5`] function.
    (Select5, <Fut1, Fut2, Fut3, Fut4, Fut5>),
}
