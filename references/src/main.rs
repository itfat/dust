fn main() {

    // Rust has a feature for using a value without transferring ownership, called references (borrowing)
    println!("Hello, world!");

    let s1 = String::from("hello");

    let mut s2 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of {} is {}", s1, len); // Since we passing reference to fun param, so we can still use it here as well

    ///////////////////////////////////

    // change_value(&s1);

    change_value_mut(&mut s2);
    println!("Updated string s2 is: {s2}");

    ///////////////////////////////////

    // let mut s = String::from("Hello");
    // let ref1 = &mut s;
    // let ref2 = &mut s;
    // println!("If there is a mutable ref to a value, there cannot be another ref to that value, {ref1}, {ref2}"); //compile time error

    ///////////////////////////////////

    // Data Race - Race condition when;
        // 1 - two or more pointer access the same data at the same time
        // 2 - at least one pointer is writing to that data value
        // 3 - there is no mechanism to synchronize access to data

    ///////////////////////////////////

    let r1 = &s2;
    let r2 = &s2;
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s2; // no problem
    println!("{}", r3);

    ///////////////////////////////////

    // Dangling pointer - a reference pointing to a mem location that has been given to someone else

    // let ref_to_nothing = dangle();
    ///////////////////////////////////

    let return_string_instead_of_ref = no_dangle();

    println!("return_string_instead_of_ref: {return_string_instead_of_ref}");


}


fn calculate_length(str: &String) -> usize { //&String - represent references, allow to refer to some value without taking its ownership
    str.len()
}

// fn change_value(str: &String) {
//     str.push_str(", world"); // references are mutable by default - so will give compile time error
// }

fn change_value_mut(str: &mut String) {
    str.push_str(", world");
}

// fn dangle()-> &String {
//     let s = String::from("Ello");

//     &s
// } // Here s goes out of scope, and hence is dropped, its mem goes away

fn no_dangle() -> String {
    let s = String::from("Ello");

    s
} // resolve problem of dangling pointer