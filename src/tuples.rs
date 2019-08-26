/// NOTE: tuples max length 12

pub fn run() {
    let person: (&str, &str, i8) = ("khanhtc", "jp", 24);

    println!("{} now on {} and is {}", person.0, person.1, person.2);
}