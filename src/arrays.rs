/// NOTE: length of arrays can not be grow

pub fn run() {
    // array ele numbers must fit with its size
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);
    println!("Value at index 0: {}", numbers[0]);

    numbers[0] = -1;
    println!("{:?}", numbers);

    // array is stack allocated data struct
    // NOTE: in fact all value in rust are stack allocated by default
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // slicing a array
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}