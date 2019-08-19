use std::fs::File;
use std::io::{ErrorKind, Read};
use std::{io, fs};

pub fn run() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

// not nested match version
fn run_other() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn run_short_ver() {
    // unwrap call panic! on Err of Result<T, E>
    let f = File::open("hello.txt").unwrap();
    // expect similar to unwrap but we can define our error message
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// error handling in use
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// shorter with (?) operator
fn read_username_from_file_1() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// chain of (?)
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// using libs :)
fn read_username_from_file_3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// Note: do not use (?) in function which not return Result<T, E>, it causes compile err
// in case of main() which return `()`, can convert its to
// main() -> Result<(), Box<dyn Error>>
// it means: () in case of Ok and Box<dyn Error> is any kind of error
