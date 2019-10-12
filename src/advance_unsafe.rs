/// NOTE: four powers of unsafe
/// - Dereference a raw pointer
/// - Call an unsafe function or method
/// - Access or modify a mutable static variable
/// - Implement an unsafe trait

/// Difference from references and smart pointers, raw pointers:
///
/// - Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
/// - Aren’t guaranteed to point to valid memory
/// - Are allowed to be null
/// - Don’t implement any automatic cleanup

pub fn run() {
    let mut num = 5;

    // immutable raw pointer
    let r1 = &num as *const i32;

    // mutable raw pointer
    let r2 = &mut num as *mut i32;

    // without raw pointer, above code doesn't even compile since it's not allowed to have
    // one mutable pointer along with immutable pointer at the same time (can cause race condition)

    unsafe {
        println!("r1 is: {}", *r1); // 5
        println!("r2 is: {}", *r2); // 5
        *r2 = 10;
        println!("r1 is: {}", *r1); // 10
        println!("r2 is: {}", *r2); // 10
        // With raw pointers, we can create a mutable pointer and an immutable pointer to the same
        // location and change data through the mutable pointer, potentially creating a data race. Be careful!
    }

    unsafe {
        dangerous();
    }

    // an immutable raw pointer point to a location on memory
    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r is: {}", *r); // segmentation fault since nothing been stored at that address
    }
}


/// Unsafe function
unsafe fn dangerous() {
    // scope inside unsafe function is safe block already, no need to add unsafe block here

    let mut num = 5;
    let r = &num as *const i32;

    println!("<Inside unsafe function> r is: {}", *r);
}

/// Creating a Safe Abstraction over Unsafe Code

use std::slice;

/// unsafe code be wrapped inside safe Rust func so that caller of this function don't care about
/// the unsafe implementation!

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    /// The function slice::from_raw_parts_mut is unsafe because it takes a raw pointer and
    /// must trust that this pointer is valid. The offset method on raw pointers is also unsafe,
    /// because it must trust that the offset location is also a valid pointer.
    /// Therefore, we had to put an unsafe block around our calls to slice::from_raw_parts_mut and offset
    /// so we could call them.
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

#[test]
fn test_split_on_mut() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}