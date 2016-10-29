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

/// A two-dimensional fixed-size array
pub struct Matrix<T> {
  mem: Vector<Option<T>>,
  pub w: usize,
  pub h: usize
}

impl<T> ResizableMemory<T> {
    /// Creates a new resizable memory
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
    /// Creates a new resizable memory with specified capacity.
    /// # Panics
    /// Panics if type is zero-sized. Zero-sized types take no space, so you
    /// shouldn't use this constructor for them.
    /// # Aborts
    /// Aborts if out of memory
    fn new_with_size(cap: usize) -> Self {
      let element_size = mem::size_of::<T>();
      assert!(element_size != 0, "Tried to use new_with_size for ZST vector!");
      let align = mem::align_of::<T>();
      let bytes = element_size * cap;
      unsafe {
        let ptr = heap::allocate(bytes, align);
        if ptr.is_null() {
          oom();
        }
        ResizableMemory {
          ptr: Unique::new(ptr as *mut _),
          cap: cap
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
              // Grow to capacity 8 immeadetly
              let bytes = 8 * element_size;
                let ptr = heap::allocate(bytes, align);
                (8, ptr)
            } else {
              // Double the size
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
    pub fn new() -> Self {
        Vector {
            mem: ResizableMemory::new(),
            len: 0
        }
    }
    /// Creates a new vector with specified capacity
    /// # Panics
    /// Panics on zero-sized types. You shouldn't be specifying capacity for
    /// vectors containing ZSTs (as they take no space!)
    /// # Aborts
    /// Aborts if out of memory
    pub fn new_with_size(cap: usize) -> Self {
      Vector {
        mem: ResizableMemory::new_with_size(cap),
        len: 0
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
    /// Insert item to given index
    pub fn insert(&mut self, index: usize, item: T) {
    	assert!(index <= self.len, "Index out of bounds!");
    	if self.mem.cap == self.len {
    		self.mem.grow();
    	}
    	unsafe {
    		if index < self.len {
    			ptr::copy(self.mem.ptr.offset(index as isize),
    				self.mem.ptr.offset(index as isize + 1),
    				self.len - index
    			);
    			ptr::write(self.mem.ptr.offset(index as isize), item);
    			self.len += 1;
    		}
    	}
    }
    /// Remove and return item from given index
    pub fn remove(&mut self, index: usize) -> T{
    	assert!(index < self.len, "Index out of bounds!");
    	unsafe {
    		self.len -= 1;
    		let result = ptr::read(self.mem.ptr.offset(index as isize));
			ptr::copy(self.mem.ptr.offset(index as isize + 1),
				self.mem.ptr.offset(index as isize),
				self.len - index
			);
			result
    	}
    }
    pub fn len(&self) -> usize {
    	self.len
    }
    pub fn get_cap(&self) -> usize {
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
    /// Also enables sorting.
    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(*self.mem.ptr, self.len) }
    }
}

impl<T> DerefMut for Vector<T> {
    /// Returns a mutable slice corresponding to the vector, magically enabling
    /// indexing
    /// Also enables sorting.
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(*self.mem.ptr, self.len) }
    }
}

impl<T: Clone> Clone for Vector<T> {
	/// Clones the vector's contents and returns a new vector with those 
	/// contents
	fn clone(&self) -> Self {
		let mut clone = Vector::new_with_size(self.len);
		for i in 0..self.len {
			clone.push(self[i].clone());
		}
		clone
	}
}

impl<T: PartialEq> PartialEq for Vector<T> {
	/// Compares vectors by their content; if this[i] == other[i] with all
	/// 0 <= i < this.len and this.len == other.len, the vectors are equal
	fn eq(&self, other: &Vector<T>) -> bool {
		if self.len != other.len {
			return false
		}
		for i in 0..self.len {
			if self[i] != other[i] {
				return false
			}
		}
		true
	}
}

impl<T: Eq> Eq for Vector<T> {}

impl<T> Matrix<T> {
	/// Creates a new matrix with given dimensions and fills it with None
	pub fn new(width: usize, height: usize) -> Self{
		let mut mem = Vector::new_with_size(width * height);
		for _ in 0..width*height {
			mem.push(None);
		}
		Matrix {
			mem: mem,
			w: width,
			h: height
		}
	}
	/// Get an item from location x, y, wrapped in option.
	pub fn get(&self, x: usize, y: usize) -> &Option<T> {
		&self.mem[y * self.w + x]
	}
	/// Sets item in location x, y to item.
	pub fn set(&mut self, x: usize, y: usize, item: T) {
		self.mem[y * self.w + x] = Some(item);
	}
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn vector_push_pop_work() {
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
    fn vector_indexing_works() {
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
    fn vector_zero_sized_types_work() {
        struct ZeroSized; // No fields = no size!
        let mut vec = Vector::new();
        for _ in 0..10 {
            vec.push(ZeroSized {});
        }
        assert_eq!(usize::max_value(), vec.get_cap());
    }

    #[test]
    fn vector_allocates_correct_amount_of_space() {
      let mut vec1 = Vector::new();
      for i in 0..10 {
        match i {
          0 => assert_eq!(0, vec1.get_cap()),
          1...8 => assert_eq!(8, vec1.get_cap()),
          9 => assert_eq!(16, vec1.get_cap()),
          _ => panic!("Something is terribly wrong with the test!")
        }
        vec1.push(i);
      }
      let mut vec2 = Vector::new_with_size(16);
      assert_eq!(16, vec2.get_cap());
      for i in 0..18 {
      	match i {
          0...16 => assert_eq!(16, vec2.get_cap()),
          17 => assert_eq!(32, vec2.get_cap()),
          _ => panic!("Something is terribly wrong with the test!")
        }
      	vec2.push(i);
      }
    }
    
    #[test]
    fn vector_sort_works() {
    	let mut vec = Vector::new();
    	for i in 0..10 {
    		vec.push(i * 7 % 5);
    	}
    	vec.sort();
    	let mut previous = vec[0];
    	for i in 1..vec.len(){
    		if previous > vec[i] {
    			panic!("Sort didn't work");
    		} else {
    			previous = vec[i];
    		}
    	}
    }
    
    #[test]
    fn vector_insert_remove_work() {
    	let mut vec = Vector::new();
        for i in 0..10 {
            vec.push(i);
        }
        vec.insert(3, 99);
        assert_eq!(2, vec[2]);
        assert_eq!(99, vec[3]);
        assert_eq!(3, vec[4]);
        assert_eq!(3, vec.remove(4));
        assert_eq!(4, vec[4]);
        assert_eq!(99, vec[3]);
    }
    
    #[test]
    fn matrix_constructor_works() {
    	let w = 7;
    	let h = 9;
    	let mat: Matrix<usize> = Matrix::new(w, h);
    	for x in 0..w {
    		for y in 0..h {
    			assert_eq!(None, *mat.get(x, y));
    		}
    	}
    }
    
    #[test]
    fn matrix_set_works() {
    	let w = 7;
    	let h = 9;
    	let mut mat = Matrix::new(w, h);
    	for x in 0..w {
    		for y in 0..h {
    			mat.set(x, y, x * 2 + y);
    		}
    	}
    	for x in 0..w {
    		for y in 0..h {
    			assert_eq!(x * 2 + y, mat.get(x, y).unwrap());
    		}
    	}
    }
}
