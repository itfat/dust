fn main() {
    let mut s = String::new();
    ///////////////////////////////////
    //Creating a string
    ///////////////////////////////////

    //Two ways to create String from string literals

    let data = "string literal";
    let s = data.to_string();
    println!("{s}");

    let s = String::from("string literal");
    println!("{s}");

    ///////////////////////////////////
    //Updating a string
    ///////////////////////////////////

    let mut s = String::from("Ello");
    s.push_str(" Wold"); //push_str does not take ownership of its param
    println!("{s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is: {s2}"); //s2 ownership isnt moved to s1, that's why its compiling perfectly
    println!("s1 is: {s1}");

    let mut s = String::from("lo");
    s.push('l'); //push pushes single character
    println!("{s}");
    
    ///////////////////////////////////
    //Concatenating a string
    ///////////////////////////////////

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; //string concatenation requires an owned `String` on the left
    println!("{s3}");
    // cause the definition of add is something like this
    // fn add(self, s: &str) -> String {}

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

    //better way to concat complicated strings is by format macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");


    ///////////////////////////////////
    //Indexing into string
    ///////////////////////////////////
    let s1 = String::from("tic");
    // let h = s1[0]; //String` cannot be indexed by `{integer}`
    //Rust strings does not allow indexing

    ///////////////////////////////////
    //Slicing strings
    ///////////////////////////////////
    let s1 = "Hello";
    let s = &s1[0..3];
    println!("{s}");


    

}
