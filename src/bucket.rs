//! Library made for fun that offers a very simple container for any value without mutation.

#[cfg(test)]
mod tests;

use libc::*;
use std::alloc::{dealloc, Layout};
use std::mem;

/// Simple container that holds any value.
///
/// **!WARNING!** <br>
/// If your value is zeroed, the "bucket" is considered empty. <br>
/// *This features probably won't be revoked.*
pub struct Bucket<T> {
    content_ptr: *mut T,
}

impl<T> Bucket<T> {
    /// Creates new instance of the *Bucket* and takes ownership of the content.
    pub fn new(content: T) -> Self {
        let bucket = Self::default();

        unsafe {
            std::ptr::write(bucket.content_ptr, content);
        }

        bucket
    }

    /// This function is used to empty the "bucket" and return the value that was inside.
    ///
    /// If the "bucket" was empty, it returns variant `None`
    ///
    /// ```rust
    /// use bucket::Bucket;
    ///
    /// let bucket = Bucket::new("value");
    /// assert!(bucket.vacate().is_some());
    ///
    /// let bucket = Bucket::default();
    /// assert!(bucket.vacate().is_none());  // Returns none, because our bucket is created without any content
    /// ```
    pub fn vacate(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let content;
        unsafe {
            content = std::ptr::read::<T>(self.content_ptr as *const _);
            memset(self.content_ptr as *mut _, 0, Self::size());
        };

        Some(content)
    }

    /// Returns a reference to the content value if the "bucket" is filled.
    ///
    /// ```rust
    /// use bucket::Bucket;
    ///
    /// let bucket = Bucket::new(5);
    /// println!("{}", bucket.peek_ref());
    /// assert!(bucket.vacate().is_some());
    /// ```
    pub fn peek_ref(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        unsafe { Some(&*self.content_ptr) }
    }

    /// Returns a mutable reference to the content value if the "bucket" is filled.
    ///
    /// ```rust
    /// use bucket::Bucket;
    ///
    /// let bucket = Bucket::new(vec![1, 2, 3]);
    /// for number in bucket.peek_mut().unwrap() {
    ///     *number += 1;
    /// }
    /// ```
    pub fn peek_mut(&self) -> Option<&mut T> {
        if self.is_empty() {
            return None;
        }

        unsafe { Some(&mut *self.content_ptr) }
    }

    /// This function is used to fill the "bucket" with a desired value. <br>
    /// If "bucket" is already filled, it returns provided value.
    ///
    /// ```rust
    /// use bucket::Bucket;
    ///
    /// let bucket = Bucket::default();
    /// bucket.fill(5);
    /// assert_eq!(bucket.fill(6), Some(6));
    /// ```
    pub fn fill(&self, content: T) -> Option<T> {
        if !self.is_empty() {
            return Some(content);
        }

        unsafe {
            std::ptr::write(self.content_ptr, content);
        }

        None
    }

    /// Checks if the "bucket" is empty *(zeroed)*.
    pub fn is_empty(&self) -> bool {
        for i in 0..Self::size() {
            let byte = (self.content_ptr as usize + i) as *const u8;
            unsafe {
                if *byte != 0 {
                    return false;
                }
            }
        }
        true
    }

    #[inline]
    const fn size() -> usize {
        mem::size_of::<T>()
    }

    fn dealloc(ptr: *mut T) {
        unsafe {
            dealloc(ptr as *mut _, Layout::new::<T>());
        }
    }
}

/// Creates an empty instance of a "bucket".
impl<T> Default for Bucket<T> {
    fn default() -> Self {
        Self {
            content_ptr: unsafe {
                let ptr = malloc(Self::size()) as *mut T;
                memset(ptr as *mut _, 0, Self::size());
                ptr
            },
        }
    }
}

/// After dropping the "bucket", deallocates previously allocated memory for the content.
impl<T> Drop for Bucket<T> {
    fn drop(&mut self) {
        Bucket::dealloc(self.content_ptr);
    }
}
