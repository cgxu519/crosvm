// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cmp::min;
use std::num::Wrapping;
use std::pin::Pin;
use std::sync::atomic::{fence, Ordering};
use std::task::{Context, Poll};

use futures::Stream;
use futures::StreamExt;

use cros_async::AsyncEventFd;
use sys_util::{error, GuestAddress, GuestMemory};

use super::VIRTIO_MSI_NO_VECTOR;

const VIRTQ_DESC_F_NEXT: u16 = 0x1;
const VIRTQ_DESC_F_WRITE: u16 = 0x2;
#[allow(dead_code)]
const VIRTQ_DESC_F_INDIRECT: u16 = 0x4;

/// An iterator over a single descriptor chain.  Not to be confused with AvailIter,
/// which iterates over the descriptor chain heads in a queue.
pub struct DescIter<'a> {
    next: Option<DescriptorChain<'a>>,
}

impl<'a> DescIter<'a> {
    /// Returns an iterator that only yields the readable descriptors in the chain.
    pub fn readable(self) -> impl Iterator<Item = DescriptorChain<'a>> {
        self.take_while(DescriptorChain::is_read_only)
    }

    /// Returns an iterator that only yields the writable descriptors in the chain.
    pub fn writable(self) -> impl Iterator<Item = DescriptorChain<'a>> {
        self.skip_while(DescriptorChain::is_read_only)
    }
}

impl<'a> Iterator for DescIter<'a> {
    type Item = DescriptorChain<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.next.take() {
            self.next = current.next_descriptor();
            Some(current)
        } else {
            None
        }
    }
}

/// A virtio descriptor chain.
#[derive(Clone)]
pub struct DescriptorChain<'a> {
    mem: &'a GuestMemory,
    desc_table: GuestAddress,
    queue_size: u16,
    ttl: u16, // used to prevent infinite chain cycles

    /// Index into the descriptor table
    pub index: u16,

    /// Guest physical address of device specific data
    pub addr: GuestAddress,

    /// Length of device specific data
    pub len: u32,

    /// Includes next, write, and indirect bits
    pub flags: u16,

    /// Index into the descriptor table of the next descriptor if flags has
    /// the next bit set
    pub next: u16,
}

impl<'a> DescriptorChain<'a> {
    pub(crate) fn checked_new(
        mem: &GuestMemory,
        desc_table: GuestAddress,
        queue_size: u16,
        index: u16,
        required_flags: u16,
    ) -> Option<DescriptorChain> {
        if index >= queue_size {
            return None;
        }

        let desc_head = match mem.checked_offset(desc_table, (index as u64) * 16) {
            Some(a) => a,
            None => return None,
        };
        // These reads can't fail unless Guest memory is hopelessly broken.
        let addr = GuestAddress(mem.read_obj_from_addr::<u64>(desc_head).unwrap() as u64);
        if mem.checked_offset(desc_head, 16).is_none() {
            return None;
        }
        let len: u32 = mem.read_obj_from_addr(desc_head.unchecked_add(8)).unwrap();
        let flags: u16 = mem.read_obj_from_addr(desc_head.unchecked_add(12)).unwrap();
        let next: u16 = mem.read_obj_from_addr(desc_head.unchecked_add(14)).unwrap();
        let chain = DescriptorChain {
            mem,
            desc_table,
            queue_size,
            ttl: queue_size,
            index,
            addr,
            len,
            flags,
            next,
        };

        if chain.is_valid() && chain.flags & required_flags == required_flags {
            Some(chain)
        } else {
            None
        }
    }

    #[allow(clippy::if_same_then_else)]
    fn is_valid(&self) -> bool {
        if self.len > 0
            && self
                .mem
                .checked_offset(self.addr, self.len as u64 - 1u64)
                .is_none()
        {
            false
        } else if self.has_next() && self.next >= self.queue_size {
            false
        } else {
            true
        }
    }

    /// Gets if this descriptor chain has another descriptor chain linked after it.
    pub fn has_next(&self) -> bool {
        self.flags & VIRTQ_DESC_F_NEXT != 0 && self.ttl > 1
    }

    /// If the driver designated this as a write only descriptor.
    ///
    /// If this is false, this descriptor is read only.
    /// Write only means the the emulated device can write and the driver can read.
    pub fn is_write_only(&self) -> bool {
        self.flags & VIRTQ_DESC_F_WRITE != 0
    }

    /// If the driver designated this as a read only descriptor.
    ///
    /// If this is false, this descriptor is write only.
    /// Read only means the emulated device can read and the driver can write.
    pub fn is_read_only(&self) -> bool {
        self.flags & VIRTQ_DESC_F_WRITE == 0
    }

    /// Gets the next descriptor in this descriptor chain, if there is one.
    ///
    /// Note that this is distinct from the next descriptor chain returned by `AvailIter`, which is
    /// the head of the next _available_ descriptor chain.
    pub fn next_descriptor(&self) -> Option<DescriptorChain<'a>> {
        if self.has_next() {
            // Once we see a write-only descriptor, all subsequent descriptors must be write-only.
            let required_flags = self.flags & VIRTQ_DESC_F_WRITE;
            DescriptorChain::checked_new(
                self.mem,
                self.desc_table,
                self.queue_size,
                self.next,
                required_flags,
            )
            .map(|mut c| {
                c.ttl = self.ttl - 1;
                c
            })
        } else {
            None
        }
    }

    /// Produces an iterator over all the descriptors in this chain.
    pub fn into_iter(self) -> DescIter<'a> {
        DescIter { next: Some(self) }
    }
}

/// Consuming iterator over all available descriptor chain heads in the queue.
pub struct AvailIter<'a, 'b> {
    mem: &'a GuestMemory,
    queue: &'b mut Queue,
}

impl<'a, 'b> Iterator for AvailIter<'a, 'b> {
    type Item = DescriptorChain<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop(self.mem)
    }
}

/// Consuming async iterator over all available descriptor chain heads in the queue.
pub struct AvailStream<'a, 'b, 'c> {
    mem: &'a GuestMemory,
    queue: &'b mut Queue,
    eventfd: &'c mut AsyncEventFd,
}

impl<'a, 'b, 'c> AvailStream<'a, 'b, 'c> {
    fn pop(&mut self) -> Option<DescriptorChain<'a>> {
        self.queue.pop(self.mem)
    }
}

impl<'a, 'b, 'c> Stream for AvailStream<'a, 'b, 'c> {
    type Item = DescriptorChain<'a>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        loop {
            // Check if there are more descriptors available.
            if let Some(chain) = self.pop() {
                return Poll::Ready(Some(chain));
            }
            // Check if the eventfd is ready and return pending if not.
            if self.eventfd.poll_next_unpin(cx) == Poll::Pending {
                return Poll::Pending;
            }
        }
    }
}

#[derive(Clone)]
/// A virtio queue's parameters.
pub struct Queue {
    /// The maximal size in elements offered by the device
    pub max_size: u16,

    /// The queue size in elements the driver selected
    pub size: u16,

    /// Inidcates if the queue is finished with configuration
    pub ready: bool,

    /// MSI-X vector for the queue. Don't care for INTx
    pub vector: u16,

    /// Guest physical address of the descriptor table
    pub desc_table: GuestAddress,

    /// Guest physical address of the available ring
    pub avail_ring: GuestAddress,

    /// Guest physical address of the used ring
    pub used_ring: GuestAddress,

    next_avail: Wrapping<u16>,
    next_used: Wrapping<u16>,
}

impl Queue {
    /// Constructs an empty virtio queue with the given `max_size`.
    pub fn new(max_size: u16) -> Queue {
        Queue {
            max_size,
            size: max_size,
            ready: false,
            vector: VIRTIO_MSI_NO_VECTOR,
            desc_table: GuestAddress(0),
            avail_ring: GuestAddress(0),
            used_ring: GuestAddress(0),
            next_avail: Wrapping(0),
            next_used: Wrapping(0),
        }
    }

    /// Return the actual size of the queue, as the driver may not set up a
    /// queue as big as the device allows.
    pub fn actual_size(&self) -> u16 {
        min(self.size, self.max_size)
    }

    pub fn is_valid(&self, mem: &GuestMemory) -> bool {
        let queue_size = self.actual_size() as usize;
        let desc_table = self.desc_table;
        let desc_table_size = 16 * queue_size;
        let avail_ring = self.avail_ring;
        let avail_ring_size = 6 + 2 * queue_size;
        let used_ring = self.used_ring;
        let used_ring_size = 6 + 8 * queue_size;
        if !self.ready {
            error!("attempt to use virtio queue that is not marked ready");
            false
        } else if self.size > self.max_size || self.size == 0 || (self.size & (self.size - 1)) != 0
        {
            error!("virtio queue with invalid size: {}", self.size);
            false
        } else if desc_table
            .checked_add(desc_table_size as u64)
            .map_or(true, |v| !mem.address_in_range(v))
        {
            error!(
                "virtio queue descriptor table goes out of bounds: start:0x{:08x} size:0x{:08x}",
                desc_table.offset(),
                desc_table_size
            );
            false
        } else if avail_ring
            .checked_add(avail_ring_size as u64)
            .map_or(true, |v| !mem.address_in_range(v))
        {
            error!(
                "virtio queue available ring goes out of bounds: start:0x{:08x} size:0x{:08x}",
                avail_ring.offset(),
                avail_ring_size
            );
            false
        } else if used_ring
            .checked_add(used_ring_size as u64)
            .map_or(true, |v| !mem.address_in_range(v))
        {
            error!(
                "virtio queue used ring goes out of bounds: start:0x{:08x} size:0x{:08x}",
                used_ring.offset(),
                used_ring_size
            );
            false
        } else {
            true
        }
    }

    /// Get the first available descriptor chain without removing it from the queue.
    /// Call `pop_peeked` to remove the returned descriptor chain from the queue.
    pub fn peek<'a>(&mut self, mem: &'a GuestMemory) -> Option<DescriptorChain<'a>> {
        if !self.is_valid(mem) {
            return None;
        }

        let queue_size = self.actual_size();
        let avail_index_addr = mem.checked_offset(self.avail_ring, 2).unwrap();
        let avail_index: u16 = mem.read_obj_from_addr(avail_index_addr).unwrap();
        let avail_len = Wrapping(avail_index) - self.next_avail;

        if avail_len.0 > queue_size || self.next_avail == Wrapping(avail_index) {
            return None;
        }

        let desc_idx_addr_offset = 4 + (u64::from(self.next_avail.0 % queue_size) * 2);
        let desc_idx_addr = mem.checked_offset(self.avail_ring, desc_idx_addr_offset)?;

        // This index is checked below in checked_new.
        let descriptor_index: u16 = mem.read_obj_from_addr(desc_idx_addr).unwrap();

        DescriptorChain::checked_new(mem, self.desc_table, queue_size, descriptor_index, 0)
    }

    /// Remove the first available descriptor chain from the queue.
    /// This function should only be called immediately following `peek`.
    pub fn pop_peeked(&mut self) {
        self.next_avail += Wrapping(1);
    }

    /// If a new DescriptorHead is available, returns one and removes it from the queue.
    pub fn pop<'a>(&mut self, mem: &'a GuestMemory) -> Option<DescriptorChain<'a>> {
        let descriptor_chain = self.peek(mem);
        if descriptor_chain.is_some() {
            self.pop_peeked();
        }
        descriptor_chain
    }

    /// A consuming iterator over all available descriptor chain heads offered by the driver.
    pub fn iter<'a, 'b>(&'b mut self, mem: &'a GuestMemory) -> AvailIter<'a, 'b> {
        AvailIter { mem, queue: self }
    }

    /// Puts an available descriptor head into the used ring for use by the guest.
    pub fn add_used(&mut self, mem: &GuestMemory, desc_index: u16, len: u32) {
        if desc_index >= self.actual_size() {
            error!(
                "attempted to add out of bounds descriptor to used ring: {}",
                desc_index
            );
            return;
        }

        let used_ring = self.used_ring;
        let next_used = (self.next_used.0 % self.actual_size()) as usize;
        let used_elem = used_ring.unchecked_add((4 + next_used * 8) as u64);

        // These writes can't fail as we are guaranteed to be within the descriptor ring.
        mem.write_obj_at_addr(desc_index as u32, used_elem).unwrap();
        mem.write_obj_at_addr(len as u32, used_elem.unchecked_add(4))
            .unwrap();

        self.next_used += Wrapping(1);

        // This fence ensures all descriptor writes are visible before the index update is.
        fence(Ordering::Release);

        mem.write_obj_at_addr(self.next_used.0 as u16, used_ring.unchecked_add(2))
            .unwrap();
    }
}
