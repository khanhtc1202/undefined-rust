// An array which can change it length

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    println!("{:?}", numbers);

    // change length of vector
    numbers.push(5);
    println!("{:?}", numbers);

    // loop
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop and modify value of elements
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("New Vector: {:?}", numbers);
}