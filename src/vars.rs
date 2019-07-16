pub fn run() {
    let name = "khanhtc";
    let mut age = 24;

    println!("Name: {} and age is {}", name, age);

    age = 25;
    println!("Name: {} and age is updated to {}", name, age);

    // constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multi
    let ( new_name, new_age ) = ("khanhtc", 24);
    println!("Name: {} and age is updated to {}", new_name, new_age);
}