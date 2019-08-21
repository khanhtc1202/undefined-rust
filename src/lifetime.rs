// Three rules rust compiler uses to inference lifetime implicitly
//
// The first rule is that each parameter that is a reference gets its own lifetime parameter.
// In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32);
// a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.
//
// The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to
// all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
//
// The third rule is if there are multiple input lifetime parameters, but one of them
// is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.

pub fn run() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    export();
}

// pass 2 reference variables to func as parameters
// (we don't want the string value to be moved here)
// 'a parameter ensure that s1, s2 and the return ref will have the same life time
// with the smaller ref ( s1 or s2 )
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// NOTE: When returning a reference from a function, the lifetime parameter for
// the return type needs to match the lifetime parameter for one of the parameters.
// If the reference returned does not refer to one of the parameters,
// it must refer to a value created within this function, which would be a dangling reference
// because the value will go out of scope at the end of the function.

// use life time parameter in struct definition to ensure instances of that type
// can't outlive the references which be used in that type
struct ImportantPart<'a> {
    part: &'a str
}

fn export() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantPart { part: first_sentence };

    println!("i.part = {}", i.part);
}

// NOTE: 'static is live long lifetime parameter. Use it when you want your reference
// live long as entire program.
// eg: let s: &'static str = "I have a static lifetime.";
// The text of this string is stored directly in the program’s binary, which is always available.
// Therefore, the lifetime of all string literals is 'static.
// WARNING: should not use 'static lifetime, your ref does not need to live that long

