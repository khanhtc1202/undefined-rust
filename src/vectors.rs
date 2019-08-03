// An array which can change it length

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    println!("{:?}", numbers);

    // change length of vector
    numbers.push(5);
    println!("{:?}", numbers);

    // loop
    // note that ref mark (&) is required here to make a borrow ref of vector
    // not move the vector to to for loop scope which cause vector be removed after finish the loop
    for x in &numbers {
        println!("Number: {}", x);
    }

    // loop iter
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop and modify value of elements
    // as above, can use `&mut numbers` instead
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("New Vector: {:?}", numbers);

    // access
    let third = &numbers[2];
    println!("Number: {}", third);

    match numbers.get(2) {
        Some(third) => println!("Number: {}", third),
        None => println!("Number not found"),
    }

    // change state of numbers vector is ok if all borrowing ref are
    // not to be call after the vector have change itself
    numbers.push(5);
    println!("Vector: {:?}", numbers);
    // so below statement will panic in compile time
    // ( it call immutable ref after vector change itself state )
    // println!("The third ele: {}", third);

    // vector of enum element
    enum Element {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row_data = vec![
        Element::Int(3),
        Element::Text(String::from("sample")),
        Element::Float(10.12)
    ];
}