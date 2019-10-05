pub fn run() {

}

fn literal() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn name_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    } // expect Matched, y = 5

    println!("at the end: x = {:?}, y = {:?}", x, y); // Some(5), 10
}

fn multiple_pattern() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn match_ranger() {
    let x = 5;

    match x {
        1...5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'...'j' => println!("early ASCII letter"),
        'k'...'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

fn match_destructure() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    ChangeColorNest(Color)
}

fn match_enum() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

fn match_nested() {
    let msg = Message::ChangeColorNest(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColorNest(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        },
        Message::ChangeColorNest(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h,
                s,
                v
            )
        }
        _ => ()
    }
}

/// NOTE ignore pattern
/// There are a few ways to ignore entire values or parts of values in a pattern:
/// using the _ pattern (which you’ve seen),
/// using the _ pattern within another pattern,
/// using a name that starts with an underscore,
/// using .. to ignore remaining parts of a value.

fn ignore_part_of_val() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    // In the first match arm, we don’t need to match on or use the values inside either Some variant,
    // but we do need to test for the case when setting_value and new_setting_value are the Some variant.
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

fn move_in_case_name_ignore_usage() {
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s); // ok since the s value not be moved to ignored val

//    // note: bellow, ownership of s be moved to _s even the _s val not being used in anywhere else. aka bind to _s
//    if let Some(_s) = s {
//        println!("found a string");
//    }
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn ignore_other_part() {
    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }
}

fn extra_condition() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // in bellow eg, y is outer y not shadowed y
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // with or ope
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

enum IdHolder {
    Hello{id: i32}
}

fn at_ope() {
    let msg = IdHolder::Hello { id: 5 };

    // By specifying `id @` before the range 3...7, we’re capturing whatever
    // value matched the range while also testing that the value matched the range pattern.
    match msg {
        IdHolder::Hello { id: id @ 3...7 } => {
            println!("Found an id in range: {}", id)
        },
        IdHolder::Hello { id: 10...12 } => {
            println!("Found an id in another range")
        },
        IdHolder::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}