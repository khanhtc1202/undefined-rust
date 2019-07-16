pub fn run() {
    greeting("hello", "khanhtc");

    // bind function values to value
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // closure
    let outside_val = 5;
    // below is closure fun which use value from outside of its scope
    let add_num = |n1: i32, n2: i32| n1 + n2 + outside_val;
    println!("Closure sum: {}", add_num(2, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    // function automatically return last expression as return value
    n1 + n2
}