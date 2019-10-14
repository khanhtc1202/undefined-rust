pub fn run() {}

/// Function pointer (fn)
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[test]
fn test_twice() {
    let answer = do_twice(add_one, 5);
    assert_eq!(12, answer);
}

/// Unlike closures, `fn` is a type rather than a trait, so we specify `fn` as the parameter type directly
/// rather than declaring a generic type parameter with one of the `Fn` traits as a trait bound.
///
/// Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce), so
/// you can always pass a function pointer as an argument for a function that expects a closure.
/// >> `It’s best to write functions using a generic type and one of the closure traits so your functions can accept either functions or closures.`

/// Return closure

/// Closure is trait and in most of case when we want to return some trait, we return concrete type which implement
/// that trait. In case of closure, there is no concrete type which impl Fn FnMut or FnOnce for us to return
/// Then remember about trait object! we use it to return closure :))
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
