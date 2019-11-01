// Copyright 2019 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod fd_executor;

pub use fd_executor::{add_future, add_waker, FdExecutor};
