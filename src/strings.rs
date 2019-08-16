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

    let mut str1 = String::from("foo");
    let str2 = "bar";
    // literal string (primitive type string) created under reference
    // so that method like push_str can take the ref to literal string and do stuff with it
    // easily
    str1.push_str(str2);
    // can print out the str2 here due to it ref passed to push_str, not the str2 itself
    println!("str2 is {}", str2);
    println!("str1 is {}", str1);

    // can concat 2 strings with (+) operator
    let str3 = str1 + str2;
    // after this line, str1 and str2 are moved and can't be called anymore
    // note: if we use: let str3 = str1 + &str2; instead, str2 is able to be used after this line
    // but str1 MUST be move here in oder to make compiler understand behavior of (+) operator
    // which be used here
    println!("str3 is {}", str3);
    // this line will caught err: println!("str1 is {}", str1);

    // concat using format!
    let s = format!("{}-{}-{}", "hello", "sample", "format");
    println!("s is {}", s);
}