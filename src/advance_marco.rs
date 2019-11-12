pub fn run() {
    Pancakes::hello_macro();
}

/// The term macro refers to a family of features in Rust: declarative macros with macro_rules! and three kinds of procedural macros:
/// - Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
/// - Attribute-like macros that define custom attributes usable on any item
/// - Function-like macros that look like function calls but operate on the tokens specified as their argument

/// Marco is metaprogramming in Rust
/// A function signature must declare the number and type of parameters the function has.
/// Macros, on the other hand, can take a variable number of parameters: we can call println!("hello")
/// with one argument or println!("hello {}", name) with two arguments.
/// Also, macros are expanded before the compiler interprets the meaning of the code, so a macro can,
/// for example, implement a trait on a given type. A function can’t, because it gets called at runtime and a trait needs to be implemented at compile time.

/// Macros with macro_rules! for General Metaprogramming
/// aka. Declarative Marco ( generate code before compile )
#[macro_export]
macro_rules! x_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[test]
fn test_vec() {
    let v = x_vec![1, 2, 3];
    assert_eq!(vec![1, 2, 3], v);
}

/// Note: more about macro found here :point_down:
/// https://danielkeep.github.io/tlborm/book/index.html



/// Procedural Macros for Generating Code from Attributes
/// aka. Procedural Macros accepts some code as an input, operate on that code, and produce some code as an output

/// The three kinds of procedural macros (custom derive, attribute-like, and function-like) all work in a similar fashion.
/// Eg:
/// use proc_macro::TokenStream;
///
/// #[some_attribute]
/// pub fn some_name(input: TokenStream) -> TokenStream {
///     TokenStream
/// }

/// Create own `derive` marco
///
/// Additionally, we can’t yet provide the hello_macro function with default implementation that will print
/// the name of the type the trait is implemented on: Rust doesn’t have reflection capabilities,
/// so it can’t look up the type’s name at runtime. We need a macro to generate code at compile time.

/// original version
pub trait HelloMacro {
    fn hello_macro();
}

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

#[test]
fn test_derive_macro() {
    Pancakes::hello_macro();
}

/// NOTE: https://doc.rust-lang.org/book/ch19-06-macros.html

/// Get marco
macro_rules! get {
      ($t:ty) => {
          {
              let mut line: String = String::new();
              std::io::stdin().read_line(&mut line).unwrap();
              line.trim().parse::<$t>().unwrap()
          }
      };
      ($($t:ty),*) => {
          {
              let mut line: String = String::new();
              std::io::stdin().read_line(&mut line).unwrap();
              let mut iter = line.split_whitespace();
              (
                  $(iter.next().unwrap().parse::<$t>().unwrap(),)*
              )
          }
      };
      ($t:ty; $n:expr) => {
          (0..$n).map(|_|
              get!($t)
          ).collect::<Vec<_>>()
      };
      ($($t:ty),*; $n:expr) => {
          (0..$n).map(|_|
              get!($($t),*)
          ).collect::<Vec<_>>()
      };
      ($t:ty ;;) => {
          {
              let mut line: String = String::new();
              std::io::stdin().read_line(&mut line).unwrap();
              line.split_whitespace()
                  .map(|t| t.parse::<$t>().unwrap())
                  .collect::<Vec<_>>()
          }
      };
      ($t:ty ;; $n:expr) => {
          (0..$n).map(|_| get!($t ;;)).collect::<Vec<_>>()
      };
}

#[test]
fn test_get_macro() {
    let (a, b) = get!(usize, usize);
    println!("{} {}", a, b);
}
