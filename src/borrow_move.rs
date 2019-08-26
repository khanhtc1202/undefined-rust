/// NOTE: type which not implemented COPY trait need call borrow explicit
/// if not, old variable (old owner of re-assigned value) will lost the ownership
/// with it data and can be use

/// NOTE 2: data type which known size will be push to stack by default
/// the one which unknown size (such as Vector, String,...) will be allocated in heap
/// if that data type do not implement COPY trait yet, make sure you borrow value across
/// re-assign operation

/// NOTE 3: custom data type (by struct keyword) which contains known size type is
/// known size type and variable of this type will be pushed to stack :)))
/// note that if that type not implement COPY trait, still need borrow explicitly :))

struct MyType {
    x: i32,
    y: i32
}

impl MyType {
    fn to_string(&self) -> String {
        format!("x = {} y = {}", self.x, self.y)
    }
}

pub fn run() {
    // can borrow explicit
    let mut a = 10;
    let b = &a;
    println!("old a = {}", a);
    println!("borrower b = {}", b);
    // change a will change b too
    a = 20;
    println!("new a = {}", a);
    // the ref to old value (aka 10) is no more valid because it been changed by the owner
    // so below statement will be marked as error by the compiler
    // println!("b = {}", b);

    // i32 is primitive type and implemented Copy trait
    // so re-assign operator COPY the value implicit
    let mut x = 10;
    let y = x;
    println!("x = {}", x);
    println!("y = {}", y);
    // change x not effect y
    x = 20;
    println!("x = {}", x);
    println!("y = {}", y);

    // MyType is known size type, m_x and m_y pushed to stack
    // but MyType not implemented COPY trait yet, so must borrow explicitly
    let m_x = MyType{ x: 2, y: 1};
    let m_y = &m_x;
    println!("m_x = {:?}", m_x.to_string());
    println!("m_y = {:?}", m_y.to_string());

    // vector allocated in heap and it not implemented COPY trait!
    // => must borrow to make both vec_x, vex_y point to [1] value
    let vec_x = vec![1];
    let vec_y = &vec_x;
    println!("vec_x = {:?}", vec_x);
    println!("vec_y = {:?}", vec_y);

    // $str store in stack but string itself stored in heap???
    let mut s = "sample";
    println!("s = {}", s);
    s = "xxxxxxxxxxxxx";
    println!("s = {}", s);

    // String type which use heap to allocate the string data
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    println!("{} and {}", r1, r2);
    // r1 and r2 are valid until we create new mutable ref

    let r3 = &mut s; // no problem
    // r1 and r2 are no more valid from here
    println!("{}", r3);

    // taking ownership be pass its into other function as its parameter
    let o_s = String::from("sample");
    println!("string in main = {}", o_s);
    sample_take_ownership(o_s);
    // value is move to parameter of above function, so o_s does not have ownership of that string
    // value => below statement will occurred error
    // println!("string in main = {}", o_s);

    // taking ownership and return it after be used inside external function
    let r_s = String::from("sample");
    println!("string in main = {}", r_s);
    let b_s = sample_take_ownership_and_return(r_s);
    println!("string in main = {}", b_s);
    // below statement is invalid because the value had been moved
    // println!("string in main = {}", r_s);

    // taking the ref to value and its ownership still belong to outside var
    let m_s = String::from("sample");
    println!("string in main = {}", m_s);
    sample_take_reference_instead_of_value(&m_s);
    println!("string in main = {}", m_s);

    // borrow mutable ref
    // both ref mut var and owner var can make change on the value
    let mut bm_s = String::from("sample");
    let rm_s = &mut bm_s;
    rm_s.push_str(" here");
    println!("string mutable ref = {}", rm_s);
    // make change the string value by bm_s after this line will be change
    // in the new string, which be changed by rm_s (when it hold the mutable ref to string)
    bm_s.push_str(" change myself");
    println!("string be changed itself = {}", bm_s);
    // after this line, if we call rm_s, it will occurred error because we try to borrow
    // &mut bm_s one more time. the &mut bm_s be borrowed once at line 107 and line 110
    // the &mut bm_s ref ownership is took over (&mut self in method impl)
    // after line 110, the mutable ref to value string belongs to bm_s itself
    // if we try to do something like below statement, it will occurred error
    // due to rm_s has no ownership to string value
    // println!("value pointed by rm_s = {}", rm_s);
}

fn sample_take_ownership(string: String) {
    println!("string inside the function = {}", string);
}

fn sample_take_ownership_and_return(string: String) -> String {
    println!("string inside the function = {}", string);
    string
}

fn sample_take_reference_instead_of_value(string_ref: &String) {
    println!("string inside the function = {}", string_ref);
}
