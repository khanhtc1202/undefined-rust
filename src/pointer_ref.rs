// Pointer ref: point to resources

pub fn run() {
    // primitive value
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    // TODO: read more about move and borrow
    // non-primitive value (not implement copy trait by default)
    // so the value will be moved after assign to other data
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1; // create ref to vec1

    println!("Values: {:?}", (&vec1, vec2));
}