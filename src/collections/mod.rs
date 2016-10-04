//! This module contains various data structures, as the assignemnt that this
//! project is for requires us to program our own instead of using the readily
//! made standard library structures. https://doc.rust-lang.org/nomicon/vec.html
//! helped a lot!

extern crate alloc;

use std::ptr::{self, Unique};
use std::{mem, slice};
use self::alloc::heap;
use std::process::exit;
use std::ops::{Deref, DerefMut};

/// Out-of-memory abort
fn oom() {
    exit(-666); //"Some random number"
}

/// A runtime-resizable piece of memory. Handles allocating and deallocating
/// space from the heap for purposes determined by other objects.
pub struct ResizableMemory<T> {
    pub ptr: Unique<T>,
    pub cap: usize,
}

/// A implementation of vector, a growable array-type list
pub struct Vector<T> {
    mem: ResizableMemory<T>,
    len: usize,
}

impl<T> ResizableMemory<T> {
    /// Creates a new resizable array
    /// # Panics
    /// Panics on zero-sized types
    fn new() -> Self {
        unsafe {
            let cap = if mem::size_of::<T>() == 0 {
                usize::max_value()
            } else {
                0
            };
            ResizableMemory {
                ptr: Unique::new(heap::EMPTY as *mut _),
                cap: cap,
            }
        }
    }
    /// Grows the memory to double the size, or size 1 if cap was 0.
    /// # Panics
    /// Panics on capacity overflow (if new capacity > isize::max_value())
    /// # Aborts
    /// Aborts if out of memory
    fn grow(&mut self) {
        unsafe {
            let element_size = mem::size_of::<T>();
            assert!(element_size != 0, "Capacity overflow");
            // If element size is 0, cap is usize::max_value() and therefore
            // getting here means that the vec is overfull.
            let align = mem::align_of::<T>();
            let (new_cap, ptr) = if self.cap == 0 {
                let ptr = heap::allocate(element_size, align);
                (1, ptr)
            } else {
                let new_cap = self.cap * 2;
                let old_bytes = self.cap * element_size;
                assert!(old_bytes <= (isize::max_value() as usize) / 2,
                        "Capacity overflow on a resizable array!");
                let new_bytes = old_bytes * 2;
                let ptr = heap::reallocate(*self.ptr as *mut _,
                                           old_bytes,
                                           new_bytes,
                                           align);
                (new_cap, ptr)
            };
            if ptr.is_null() {
                oom();
            }
            self.ptr = Unique::new(ptr as *mut _);
            self.cap = new_cap;
        }
    }
}

impl<T> Drop for ResizableMemory<T> {
    /// Frees the memory, doesn't try to drop the contents
    fn drop(&mut self) {
        let element_size = mem::size_of::<T>();
        if self.cap != 0 && element_size != 0 {
            // Zero-sized allocations won't be freed
            // (as nothing has been allocated in the first place)
            let align = mem::align_of::<T>();
            let bytes = element_size * self.cap;
            unsafe {
                heap::deallocate(*self.ptr as *mut _, bytes, align);
            }
        }
    }
}

impl<T> Vector<T> {
    /// Creates a new vector with zero capacity
    fn new() -> Self {
        Vector {
            mem: ResizableMemory::new(),
            len: 0,
        }
    }
    /// Pushes an item to the end of the vec
    /// # Panics
    /// Panics on capacity overflow (if new capacity > isize::MAX)
    /// # Aborts
    /// Aborts if out of memory
    pub fn push(&mut self, item: T) {
        if self.len == self.mem.cap {
            self.mem.grow();
        }
        unsafe {
            ptr::write(self.mem.ptr.offset(self.len as isize), item);
        }
        self.len += 1;
    }
    /// Removes an item from the end of the vec and returns it wrapped in an
    /// Option. Returns None if there are no items to return.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.mem.ptr.offset(self.len as isize))) }
        }
    }

    fn get_cap(&self) -> usize {
        self.mem.cap
    }
}

impl<T> Drop for Vector<T> {
    /// Drops all contents by popping them
    /// Dropping the vec automatically also drops the contained resizable memory
    fn drop(&mut self) {
        while self.len > 0 {
            let _ = self.pop();
        }
    }
}

impl<T> Deref for Vector<T> {
    type Target = [T];
    /// Returns a slice corresponding to the vector, magically enabling indexing
    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(*self.mem.ptr, self.len) }
    }
}

impl<T> DerefMut for Vector<T> {
    /// Returns a mutable slice corresponding to the vector, magically enabling
    /// indexing
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(*self.mem.ptr, self.len) }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn push_pop_work() {
        let mut vec = Vector::new();
        for i in 0..10 {
            vec.push(i);
        }
        for i in 0..11 {
            match i {
                10 => assert_eq!(None, vec.pop()),
                _ => assert_eq!(9 - i, vec.pop().unwrap()),
            }
        }
    }

    #[test]
    fn indexing_works() {
        let mut vec = Vector::new();
        for i in 0..10 {
            vec.push(i * 2);
        }
        for i in 0..10 {
            assert_eq!(i * 2, vec[i]);
        }
        for i in 0..10 {
            vec[i] = i + 1;
        }
        for i in 0..10 {
            assert_eq!(i + 1, vec[i]);
        }
    }

    #[test]
    fn zero_sized_types_work() {
        struct ZeroSized; // No fields = no size!
        let mut vec = Vector::new();
        for _ in 0..10 {
            vec.push(ZeroSized {});
        }
        assert_eq!(usize::max_value(), vec.get_cap());
    }
}
