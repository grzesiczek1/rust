// The main tools within the Rust ecosystem are:
// - rustc The compiler which takes your Rust code and compiles it into binary (machine readable code)
// - rustup The command line utility to install and update Rust
// - cargo The Rust build system and package manager (you will work with this)

fn main() {
    basic_strings();
    mutation();
    unicode_example();
}

// - Macros are called using a bang (!)
// - Macros can take a variable number of arguments; functions in Rust cannot

/* It is convention in Rust to use snake_case for: Variable names, Function names, File names
SCREAMING_SNAKE_CASE is used for constants and statics. 
PascalCase is used for types, traits, and enums (we will cover these later).
*/

fn basic_strings() {
  let first_name = String::from("Greg ").to_owned() + "Osinski";
  let name = &first_name;

  println!("Hello, {}!", name);
  println!("Hello, {}!", first_name);
}

fn mutation() {
    let mut first_name = String::from("Greg ");
    first_name.push_str("Osinski");

    println!("Hello, {}!", first_name);
}

fn unicode_example() {
    let unicode = "∞";

    println!("Lenght:{} \nCount of chars:{}",unicode.len(),unicode.chars().count())
}