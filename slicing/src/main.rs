fn main() {
    println!("Hello, world!");
    let s = String::from("Hello World");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("Hello is: {hello} and World is: {world}");

    ///////////////////////////////////

    let slice1 = &s[0..2];
    let slice2 = &s[..2];

    println!("slice1 and slice2 are the same: {slice1}, {slice2}");

    let len = s.len();
    let slice1 = &s[2..len];
    let slice2 = &s[2..];

    println!("slice1 and slice2 are the same: {slice1}, {slice2}");

    let slice1 = &s[0..len];
    let slice2 = &s[..];

    println!("slice1 and slice2 are the same: {slice1}, {slice2}");

    ///////////////////////////////////

    println!("First word is: {}", first_word(&s));

    ///////////////////////////////////

    let a = [1, 2, 3, 4, 5];
    println!("A is: {}", a.len());
    let slice = &a[1..3];
    println!("Slice is: {}", slice.len());
}

fn first_word(s: &str) -> &str { //str is string slice (different from String)
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s
}
