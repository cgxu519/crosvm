// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! Track memory regions that are mapped to the guest VM.

use std::io::{Read, Write};
use std::result;
use std::sync::Arc;

use guest_address::GuestAddress;
use mmap::MemoryMapping;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    InvalidGuestAddress(GuestAddress),
    MemoryMappingFailed,
    MemoryRegionOverlap,
    NoMemoryRegions,
    RegionOperationFailed,
}
pub type Result<T> = result::Result<T, Error>;

struct MemoryRegion {
    mapping: MemoryMapping,
    guest_base: GuestAddress,
}

/// Tracks a memory region and where it is mapped in the guest.
#[derive(Clone)]
pub struct GuestMemory {
    regions: Arc<Vec<MemoryRegion>>,
}

impl GuestMemory {
    /// Creates the container for guest memory regions.
    /// Valid memory regions are specified as a Vec of (Address, Size) tuples sorted by Address.
    pub fn new(ranges: &[(GuestAddress, usize)]) -> Result<GuestMemory> {
        if ranges.is_empty() {
            return Err(Error::NoMemoryRegions);
        }

        let mut regions = Vec::<MemoryRegion>::new();
        for range in ranges.iter() {
            if let Some(last) = regions.last() {
                if range.0 < last.guest_base + last.mapping.size() {
                    return Err(Error::MemoryRegionOverlap);
                }
            }

            let mapping = MemoryMapping::new(range.1).map_err(|_| Error::MemoryMappingFailed)?;
            regions.push(MemoryRegion {
                             mapping: mapping,
                             guest_base: range.0,
                         });
        }

        Ok(GuestMemory { regions: Arc::new(regions) })
    }

    /// Returns the end address of memory.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sys_util::{GuestAddress, GuestMemory, MemoryMapping};
    /// # fn test_end_addr() -> Result<(), ()> {
    ///     let start_addr = GuestAddress::new(0x1000);
    ///     let mut gm = GuestMemory::new(&vec![(start_addr, 0x400)]).map_err(|_| ())?;
    ///     assert_eq!(start_addr + 0x400, gm.end_addr());
    ///     Ok(())
    /// # }
    pub fn end_addr(&self) -> GuestAddress {
        self.regions
            .iter()
            .max_by_key(|region| region.guest_base)
            .map_or(GuestAddress::new(0),
                    |region| region.guest_base + region.mapping.size())
    }

    /// Returns true if the given address is within the memory range available to the guest.
    pub fn address_in_range(&self, addr: GuestAddress) -> bool {
        addr < self.end_addr()
    }

    /// Returns the size of the memory region in bytes.
    pub fn num_regions(&self) -> usize {
        self.regions.len()
    }

    /// Preform the specified action on each region's addresses.
    pub fn with_regions<F>(&self, cb: F) -> Result<()>
        where F: Fn(usize, GuestAddress, usize, usize) -> result::Result<(), ()>
    {
        for (index, region) in self.regions.iter().enumerate() {
            let host_addr = unsafe {
                // Accessing this as a pointer would be unsafe, using the address as a value is OK.
                region.mapping.as_ptr() as usize
            };
            cb(index, region.guest_base, region.mapping.size(), host_addr)
                .map_err(|_| Error::RegionOperationFailed)?;
        }
        Ok(())
    }

    /// Writes a slice to guest memory at the specified guest address.
    /// Returns Ok(<number of bytes written>).  The number of bytes written can
    /// be less than the length of the slice if there isn't enough room in the
    /// memory region.
    ///
    /// # Examples
    /// * Write a slice at guestaddress 0x200.
    ///
    /// ```
    /// # use sys_util::{GuestAddress, GuestMemory, MemoryMapping};
    /// # fn test_write_u64() -> Result<(), ()> {
    /// #   let start_addr = GuestAddress::new(0x1000);
    /// #   let mut gm = GuestMemory::new(&vec![(start_addr, 0x400)]).map_err(|_| ())?;
    ///     let res = gm.write_slice_at_addr(&[1,2,3,4,5], GuestAddress::new(0x200));
    ///     assert_eq!(Ok(5), res);
    ///     Ok(())
    /// # }
    /// ```
    pub fn write_slice_at_addr(&self, buf: &[u8], guest_addr: GuestAddress) -> Result<usize> {
        self.do_in_region(guest_addr, move |mapping, offset| {
            mapping
                .write_slice(buf, offset)
                .map_err(|_| Error::InvalidGuestAddress(guest_addr))
        })
    }

    /// Reads an object from guest memory at the given guest address.
    /// Reading from a volatile area isn't strictly safe as it could change
    /// mid-read.  However, as long as the type T is plain old data and can
    /// handle random initialization, everything will be OK.
    ///
    /// # Examples
    /// * Read a u64 from two areas of guest memory backed by separate mappings.
    ///
    /// ```
    /// # use sys_util::{GuestAddress, GuestMemory, MemoryMapping};
    /// # fn test_read_u64() -> Result<u64, ()> {
    /// #     let start_addr1 = GuestAddress::new(0x0);
    /// #     let start_addr2 = GuestAddress::new(0x400);
    /// #     let mut gm = GuestMemory::new(&vec![(start_addr1, 0x400), (start_addr2, 0x400)])
    /// #         .map_err(|_| ())?;
    ///       let num1: u64 = gm.read_obj_from_addr(GuestAddress::new(32)).map_err(|_| ())?;
    ///       let num2: u64 = gm.read_obj_from_addr(GuestAddress::new(0x400+32)).map_err(|_| ())?;
    /// #     Ok(num1 + num2)
    /// # }
    /// ```
    pub fn read_obj_from_addr<T: Copy>(&self, guest_addr: GuestAddress) -> Result<T> {
        self.do_in_region(guest_addr, |mapping, offset| {
            mapping
                .read_obj(offset)
                .map_err(|_| Error::InvalidGuestAddress(guest_addr))
        })
    }

    /// Writes an object to the memory region at the specified guest address.
    /// Returns Ok(()) if the object fits, or Err if it extends past the end.
    ///
    /// # Examples
    /// * Write a u64 at guest address 0x1100.
    ///
    /// ```
    /// # use sys_util::{GuestAddress, GuestMemory, MemoryMapping};
    /// # fn test_write_u64() -> Result<(), ()> {
    /// #   let start_addr = GuestAddress::new(0x1000);
    /// #   let mut gm = GuestMemory::new(&vec![(start_addr, 0x400)]).map_err(|_| ())?;
    ///     gm.write_obj_at_addr(55u64, GuestAddress::new(0x1100))
    ///         .map_err(|_| ())
    /// # }
    /// ```
    pub fn write_obj_at_addr<T>(&self, val: T, guest_addr: GuestAddress) -> Result<()> {
        self.do_in_region(guest_addr, move |mapping, offset| {
            mapping
                .write_obj(val, offset)
                .map_err(|_| Error::InvalidGuestAddress(guest_addr))
        })
    }

    /// Reads data from a readable object like a File and writes it to guest memory.
    ///
    /// # Arguments
    /// * `guest_addr` - Begin writing memory at this offset.
    /// * `src` - Read from `src` to memory.
    /// * `count` - Read `count` bytes from `src` to memory.
    ///
    /// # Examples
    ///
    /// * Read bytes from /dev/urandom
    ///
    /// ```
    /// # use sys_util::{GuestAddress, GuestMemory, MemoryMapping};
    /// # use std::fs::File;
    /// # use std::path::Path;
    /// # fn test_read_random() -> Result<u32, ()> {
    /// #     let start_addr = GuestAddress::new(0x1000);
    /// #     let gm = GuestMemory::new(&vec![(start_addr, 0x400)]).map_err(|_| ())?;
    ///       let mut file = File::open(Path::new("/dev/urandom")).map_err(|_| ())?;
    ///       let addr = GuestAddress::new(0x1010);
    ///       gm.read_to_memory(addr, &mut file, 128).map_err(|_| ())?;
    ///       let rand_val: u32 = gm.read_obj_from_addr(addr + 8).map_err(|_| ())?;
    /// #     Ok(rand_val)
    /// # }
    /// ```
    pub fn read_to_memory<F>(&self,
                             guest_addr: GuestAddress,
                             src: &mut F,
                             count: usize)
                             -> Result<()>
        where F: Read
    {
        self.do_in_region(guest_addr, move |mapping, offset| {
            mapping
                .read_to_memory(offset, src, count)
                .map_err(|_| Error::InvalidGuestAddress(guest_addr))
        })
    }

    /// Writes data from memory to a writable object.
    ///
    /// # Arguments
    /// * `guest_addr` - Begin reading memory from this offset.
    /// * `dst` - Write from memory to `dst`.
    /// * `count` - Read `count` bytes from memory to `src`.
    ///
    /// # Examples
    ///
    /// * Write 128 bytes to /dev/null
    ///
    /// ```
    /// # use sys_util::{GuestAddress, GuestMemory, MemoryMapping};
    /// # use std::fs::File;
    /// # use std::path::Path;
    /// # fn test_write_null() -> Result<(), ()> {
    /// #     let start_addr = GuestAddress::new(0x1000);
    /// #     let gm = GuestMemory::new(&vec![(start_addr, 0x400)]).map_err(|_| ())?;
    ///       let mut file = File::open(Path::new("/dev/null")).map_err(|_| ())?;
    ///       let addr = GuestAddress::new(0x1010);
    ///       gm.write_from_memory(addr, &mut file, 128).map_err(|_| ())?;
    /// #     Ok(())
    /// # }
    /// ```
    pub fn write_from_memory<F>(&self,
                                guest_addr: GuestAddress,
                                dst: &mut F,
                                count: usize)
                                -> Result<()>
        where F: Write
    {
        self.do_in_region(guest_addr, move |mapping, offset| {
            mapping
                .write_from_memory(offset, dst, count)
                .map_err(|_| Error::InvalidGuestAddress(guest_addr))
        })
    }

    fn do_in_region<F, T>(&self, guest_addr: GuestAddress, cb: F) -> Result<T>
        where F: FnOnce(&MemoryMapping, usize) -> Result<T>
    {
        for region in self.regions.iter() {
            if guest_addr >= region.guest_base &&
               guest_addr < (region.guest_base + region.mapping.size()) {
                return cb(&region.mapping, guest_addr.offset_from(region.guest_base));
            }
        }
        Err(Error::InvalidGuestAddress(guest_addr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_regions() {
        let start_addr1 = GuestAddress::new(0x0);
        let start_addr2 = GuestAddress::new(0x400);
        assert!(GuestMemory::new(&vec![(start_addr1, 0x400), (start_addr2, 0x400)]).is_ok());
    }

    #[test]
    fn overlap_memory() {
        let start_addr1 = GuestAddress::new(0x0);
        let start_addr2 = GuestAddress::new(0x1000);
        assert!(GuestMemory::new(&vec![(start_addr1, 0x2000), (start_addr2, 0x2000)]).is_err());
    }

    #[test]
    fn test_read_u64() {
        let start_addr1 = GuestAddress::new(0x0);
        let start_addr2 = GuestAddress::new(0x1000);
        let gm = GuestMemory::new(&vec![(start_addr1, 0x1000), (start_addr2, 0x1000)]).unwrap();

        let val1: u64 = 0xaa55aa55aa55aa55;
        let val2: u64 = 0x55aa55aa55aa55aa;
        gm.write_obj_at_addr(val1, GuestAddress::new(0x500))
            .unwrap();
        gm.write_obj_at_addr(val2, GuestAddress::new(0x1000 + 32))
            .unwrap();
        let num1: u64 = gm.read_obj_from_addr(GuestAddress::new(0x500)).unwrap();
        let num2: u64 = gm.read_obj_from_addr(GuestAddress::new(0x1000 + 32))
            .unwrap();
        assert_eq!(val1, num1);
        assert_eq!(val2, num2);
    }
}