//lifetimes ensure that references are valid as long as we need them to be
//every reference in Rust has a lifetime, which is the scope for which that reference is valid
//The main aim of lifetimes is to prevent dangling references
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime
fn main() {
    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }

    // println!("r: {}", r); //here r would be pointing mem that got deallocated when x goes out of scope
    //Rust compiler has a borrow checker that compares scopes to determine all borrows are valid 

    //Fix would be
    let x =5;
    let r = &x;
    println!("r: {}", r);

    //Generic lifetimes in functions
    let str1 = String::from("am i longer");
    let str2 = "yes";

    let result = longest(str1.as_str(), str2);
    println!("The longest string is {}", result);
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str { //Compile time error - expected named lifetime param, return type needs a generic lifetime parameter on it because Rust can’t tell whether the reference being returned refers to x or y
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}//lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the references passed in
