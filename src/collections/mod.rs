//! This module contains various data structures, as the assignemnt that this
//! project is for requires us to program our own instead of using the readily
//! made standard library structures. https://doc.rust-lang.org/nomicon/vec.html
//! helped a lot!

extern crate alloc;

use std::ptr::{Unique, self};
use std::mem;
use self::alloc::heap;
use std::process::exit;

/// Out-of-memory abort
fn oom() {
	exit(-666); //"Some random number"
}

/// A runtime-resizable array
pub struct ResizableArray<T> {
	ptr: Unique<T>,
	cap: usize,
	len: usize
}

impl<T> ResizableArray<T> {
	/// Creates a new resizable array
	/// # Panics
	/// Panics on zero-sized types
	fn new() -> Self {
		assert!(mem::size_of::<T>() != 0, "Zero-sized types not supported yet!");
		unsafe {
			ResizableArray {
				ptr: Unique::new(heap::EMPTY as *mut _),
				cap: 0,
				len: 0
			}
		}
	}
	/// Grows the array to double the size, or size 1 if cap was 0.
	/// # Panics
	/// Panics on capacity overflow (if new capacity > isize::max_value())
	/// # Aborts
	/// Aborts if out of memory
	fn grow(&mut self){
		unsafe {
			let align = mem::align_of::<T>();
			let element_size = mem::size_of::<T>();
			let (new_cap, ptr) = if self.cap == 0 {
				let ptr = heap::allocate(element_size, align);
				(1, ptr)
			} else {
				let new_cap = self.cap * 2;
				let old_bytes = self.cap * element_size;
				assert!(old_bytes <= (isize::max_value() as usize) / 2,
					"Capacity overflow on a resizable array!"
				);
				let new_bytes = old_bytes * 2;
				let ptr = heap::reallocate(*self.ptr as *mut _,
					old_bytes,
					new_bytes,
					align
				);
				(new_cap, ptr)
			};
			if ptr.is_null() { 
				oom();
			}
			self.ptr = Unique::new(ptr as *mut _);
			self.cap = new_cap;
		}
	}
	/// Pushes an item to the end of the array
	/// # Panics
	/// Panics on capacity overflow (if new capacity > isize::MAX)
	/// # Aborts
	/// Aborts if out of memory
	pub fn push(&mut self, item: T) {
		if self.len == self.cap {
			self.grow();
		}
		unsafe {
			ptr::write(self.ptr.offset(self.len as isize), item);
		}
		self.len += 1;
	}
	/// Removes an item from the end of the array and returns it wrapped in an
	/// option. Returns None if there are no items to return.
	pub fn pop(&mut self) -> Option<T> {
		if self.len == 0 {
			None
		} else {
			self.len -= 1;
			unsafe {
				Some(ptr::read(self.ptr.offset(self.len as isize)))
			}
		}
	}
}

#[cfg(test)]
mod tests {
	
	use super::*;
	
	#[test]
	fn push_pop_work() {
		let mut array = ResizableArray::new();
		for i in 0..10 {
			array.push(i);
		}
		for i in 0..11 {
			match i {
				10 => assert_eq!(None, array.pop()),
				_ => assert_eq!(9 - i, array.pop().unwrap())
			}
		}
	}
}