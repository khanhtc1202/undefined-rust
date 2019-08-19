use std::collections::HashMap;

pub fn run() {
    let mut scores = HashMap::new();

    scores.insert(String::from("A"), 10);
    scores.insert(String::from("B"), 50);

    println!("map is {:?}", scores);

    let names = vec![String::from("s1"), String::from("s2")];
    let init_scores = vec![10, 50];

    // The type annotation HashMap<_, _> is needed here because it’s possible to collect into
    // many different data structures and Rust doesn’t know which you want unless you specify.
    let scores: HashMap<_,_> = names.iter().zip(init_scores.iter()).collect();
    println!("map is {:?}", scores);

    // String and other types which NOT implement Copy trait will be moved when it be added to HashMap
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    println!("map is {:?}", map);

    // i32 is primitive type and be copied by default when it be added to HashMap
    let key = 10;
    let value = 20;
    let mut map = HashMap::new();
    map.insert(key, value);
    println!("map is {:?}", map);
    println!("key is {}", key);

    // Note: If we insert references to values into the hash map, the values won’t be moved into the hash map.
    // The values that the references point to must be valid for at least as long as the hash map is valid.

    // get value by key require &keyType as params and return an Option<T> ( None in case do not have that key )
    // use match or it-let pattern to handle the Some<T> type of return value
    let v = map.get(&key);
    match v {
        Some(v) => println!("value at {} is {}", key, v),
        _ => ()
    }

    if let Some(v) = map.get(&key) {
        println!("value at {} is {}", key, v);
    }

    // otherwise, use for loop to access <k,v> pairs in hashmap one by one
    for (k, v) in &map {
        println!("value at {} is {}", k, v);
    }

    // insert elements to hash map ( 3 ways )
    let mut scores = HashMap::new();
    scores.insert(String::from("A"), 10);

    // replace old value associated with key by the new one
    scores.insert(String::from("A"), 15);
    println!("current map is {:?}", scores); // expected value = 15

    // using entry to insert on not exist, otherwise not
    scores.entry(String::from("A")).or_insert(20);
    println!("current map is {:?}", scores); // expected value = 15

    // the mutate ref returned by or_insert fn can be used to update value pointed by it
    let v = scores.entry(String::from("A")).or_insert(0);
    *v = 20;
    println!("current map is {:?}", scores); // expected value = 20
}