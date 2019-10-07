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

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}