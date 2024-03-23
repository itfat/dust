fn main() {
    println!("Hello, world!");

    let s1 = String::from("hello");
    let s2 = s1; // This may cause double free error when scope of both varibales is invalid

    println!("The firs tstring s1 is: {s1}"); // Rust ignores s1 after it is assigned (considers it invalid) - so to avoid the above problem - this is resulting in compile time error

    //let s2 = s1; this is not shallow copy but move in Rust lang, means the first variable isnt valid anymore - value of first variable has moved to second

    // For creating a deep copy of data stored on heap, we need to use clone method
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 is: {s1} and s2 is: {s2}"); // s1 and s2 both have seperate copies of same data on heap

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y); // here x isnt invalidated because this data type is stored on stack itself, y inst moved to x, both are valid


    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function and so is no longer valid here
    println!("is s no longer valid after being passed to a function as a param: {s}"); // yes - value borrowed here after move

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function, but i32 is Copy, so it's okay to still use x afterward

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it.