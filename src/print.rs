pub fn run() {
    println!("Hello from print.rs file");

    // basic
    println!("Number: {}", 1);

    // position
    println!("Multi time args {0} and {0}", "this");

    // named
    println!("Named arg: {name}", name="this");

    // traits
    println!("Binary: {:b} Hex: {:x} Oct: {:o}", 10, 10, 10);

    // for debug traits
    println!("{:?}", (12, true, "string"));
}