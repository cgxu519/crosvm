// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::mem;
use libc::{tm, time_t, time, gmtime_r};

use hw::BusDevice;

const INDEX_MASK: u8 = 0x7f;
const INDEX_OFFSET: u64 = 0x0;
const DATA_OFFSET: u64 = 0x1;
const DATA_LEN: usize = 128;

/// A CMOS/RTC device commonly seen on x86 I/O port 0x70/0x71.
pub struct Cmos {
    index: u8,
    data: [u8; DATA_LEN],
}

impl Cmos {
    /// Constructs a CMOS/RTC device with zero data.
    pub fn new() -> Cmos {
        Cmos {
            index: 0,
            data: [0; DATA_LEN],
        }
    }
}

impl BusDevice for Cmos {
    fn write(&mut self, offset: u64, data: &[u8]) {
        if data.len() != 1 {
            return;
        }

        match offset {
            INDEX_OFFSET => self.index = data[0] & INDEX_MASK,
            DATA_OFFSET => self.data[self.index as usize] = data[0],
            _ => {}
        }
    }

    fn read(&mut self, offset: u64, data: &mut [u8]) {
        fn to_bcd(v: u8) -> u8 {
            ((v / 10) << 4) | (v % 10)
        }

        if data.len() != 1 {
            return;
        }

        data[0] = match offset {
            INDEX_OFFSET => self.index,
            DATA_OFFSET => {
                let seconds;
                let minutes;
                let hours;
                let week_day;
                let day;
                let month;
                let year;
                unsafe {
                    let mut tm: tm = mem::zeroed();
                    let mut now: time_t = 0;
                    time(&mut now as *mut _);
                    gmtime_r(&now, &mut tm as *mut _);
                    seconds = tm.tm_sec;
                    minutes = tm.tm_min;
                    hours = tm.tm_hour;
                    week_day = tm.tm_wday + 1;
                    day = tm.tm_mday;
                    month = tm.tm_mon + 1;
                    year = tm.tm_year;
                };
                match self.index {
                    0x00 => to_bcd(seconds as u8),
                    0x02 => to_bcd(minutes as u8),
                    0x04 => to_bcd(hours as u8),
                    0x06 => to_bcd(week_day as u8),
                    0x07 => to_bcd(day as u8),
                    0x08 => to_bcd(month as u8),
                    0x09 => to_bcd(year as u8),
                    0x32 => to_bcd(((year + 1900) / 100) as u8),
                    _ => {
                        match self.data.get(self.index as usize) {
                            Some(&v) => v,
                            None => return,
                        }
                    }
                }
            }
            _ => return,
        }
    }
}