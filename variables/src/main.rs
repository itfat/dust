fn main() {

    const SECONDS_IN_THREE_HOURS: u32 = 60 * 60 * 3; //const requires to have data type and is immutable by default
    println!("Seconds in 3 hours: {SECONDS_IN_THREE_HOURS}");

    ///////////////////////////////////

    let mut x = 3;
    println!("The value of x is: {x}");

    x = 4;
    println!("The value of x is: {x}");

    ///////////////////////////////////

    let y = 2;

    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in inner scope is: {y}");
    } 

    println!("The value of y is: {y}");

    ///////////////////////////////////

    let spaces = "     ";
    let spaces = spaces.len();
    println!("Spaces type has change using a single variable: {spaces}");

    let mut spaces = "     ";
    spaces = spaces.len(); // Compile-time error - mut does not allow for type change
}
