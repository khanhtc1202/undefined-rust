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

    // an immutable raw pointer point to a location on memory
    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r is: {}", *r); // segmentation fault since nothing been stored at that address
    }
}