pub fn run() {
    // primitive string (allocated in stack ???), it's immutable
    // note: mark something with underscore (_) char make it ignore by compiler for
    // checking it been used or not
    let _hello_s = "Hello string";

    // String type allocated in heap, length growable (using mut keyword to make it mutable)
    let mut hello_t = String::from("Hello string type");

    hello_t.push('s');
    hello_t.push_str(" more than this");

    println!("{}", hello_t);

    // capacity count included the '\0' (EOF) char
    println!("Length: {} Capacity: {}", hello_t.len(), hello_t.capacity());

    // other apis
    println!("Is empty: {}", hello_t.is_empty());
    println!("Contain 'world'?: {}", hello_t.contains("world"));
    println!("Replace: {}", hello_t.replace("Hello", "Hi"));

    // iterator
    for word in hello_t.split_whitespace() {
        println!("{}", word);
    }

    // new String with specific capacity
    let mut s = String::with_capacity(5);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // assert capacity and len
    assert_eq!(2, s.len());
    assert_eq!(5, s.capacity());
}