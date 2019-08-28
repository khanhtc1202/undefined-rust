use std::thread;
use std::time::Duration;
use std::collections::HashMap;

pub fn run() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}


/// Normal method: normal version which call function in case we need it results
/// Prob: we must wait for expensive cal more than one ( which actually need just once )
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} push-ups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} sit-ups!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

/// Using val method: refactor by using a val which store value from expensive cal
/// Prob: we must run the expensive cal and wait for its results even if we don't need it
/// ( in case else and inner if random number is 3 ), we do not use the results from expensive cal
fn generate_workout_one(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} push-ups!",
            expensive_result
        );
        println!(
            "Next, do {} sit-ups!",
            expensive_result
        );
    } else {
        if random_number == 3 {
            // we do not use the results of expensive cal here
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result
            );
        }
    }
}

/// closure method: use closures to store code
fn generate_workout_two(intensity: u32, random_number: u32) {
    /// closure definition: we don't need to specific type annotation here
    /// alternative name: anonymous function (but not define in function way :^), they do not have name nor signature )
    /// Since we do not specific type on closure definition, Rust compiler will infer type for us
    /// but, if you call the closure twice with difference types, rust compiler will raise err
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} push-ups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} sit-ups!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}

/// Memoization method ( or lazy evaluation method )
/// create cache struct where we store the value after first call to expensive func
struct Memo<T> where T: Fn(u32) -> u32 {
    calculation: T,
    values: HashMap<u32, u32>
}

impl<T> Memo<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Memo<T> {
        Memo {
            calculation,
            values: HashMap::new()
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        if let Some(v) = self.values.get(&arg) {
            *v
        } else {
            let v = (self.calculation)(arg);
            self.values.insert(arg, v);
            v
        }
    }
}

fn generate_workout_memo(intensity: u32, random_number: u32) {
    let mut expensive_result = Memo::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} push-ups!",
            expensive_result.value(intensity) // closure execution just be called once here
        );
        println!(
            "Next, do {} sit-ups!",
            expensive_result.value(intensity) // value from memorize
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity) // value from memorize
            );
        }
    }
}

#[test]
#[allow(dead_code)]
fn call_memo() {
    let mut c = Memo::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}


/// NOTE: Main difference between function and closure:
/// closures can capture their environment and access variables from the scope in which they’re defined.
/// mean they can use variables from parent scope :)
///
/// Closures can capture values from their environment in three ways, which directly map to the three ways
/// a function can take a parameter: taking ownership, borrowing mutably, and borrowing immutably.
/// These are encoded in the three Fn traits as follows:
///
///   FnOnce:
///     consumes the variables it captures from its enclosing scope, known as the closure’s environment.
///     To consume the captured variables, the closure must take ownership of these variables and move them
///     into the closure when it is defined. The Once part of the name represents the fact that the closure
///     can’t take ownership of the same variables more than once, so it can be called only once.
///   FnMut:
///     can change the environment because it mutably borrows values.
///   Fn:
///     borrows values from the environment immutably.
///
/// To force closure take ownership of outer var, using `move` keyword.
/// This technique is mostly useful when passing a closure to a new thread to move the data so it’s owned by the new thread.
///
/// Eg:
/// {
///    let x = vec![1, 2, 3];
///
///    let equal_to_x = move |z| z == x;
///
///    println!("can't use x here: {:?}", x); // x moved into closure
///
///    let y = vec![1, 2, 3];
///
///    assert!(equal_to_x(y));
/// }
///
/// Trick: Most of the time when specifying one of the Fn trait bounds,
/// you can start with `Fn` and the compiler will tell you if you need `FnMut` or `FnOnce` based on what happens in the closure body.
