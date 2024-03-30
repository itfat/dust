fn main() {
    let v:Vec<i32> = Vec::new(); // empty vector - told compiler type cause there's no element inside to guess type

    let v = vec![1, 2, 3]; //vec! macro - creates a vector with values provided

    //Updating vector using push

    let mut v = Vec::new(); // not given any type - it will infer from elements, mut if want to change elements of vector
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    //Reading vectors

    let v = vec![1, 2, 3, 3, 4];
    let third = &v[2];
    println!("The third element is: {third}");

    let third: Option<&i32> = v.get(7); //handles index outta range
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // let not_exist = &v[100]; // programs gets into panic state
    let not_exist = v.get(100); // Returns none without panicking

    let mut v = vec![1, 2, 3, 3, 4];
    let first = &v[0];
    v.push(6);

    // println!("The first element is: {first}"); // immutable borrow later used here
    //But why should first element reference care in case of change in vector at last index. 
    //Reason is vector takes contagious mem, if mem at current palce doesnt allow for adding another element, it has to realloc mem on some other bigger plaxe
    //For that, the first will be popinting to dealloc mem, that's why Rust'borrowing rule is preventing from ending up in that situation 

    //Iterating over vectors

    let v = vec![1, 2, 3, 3, 4];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![1, 2, 3, 3, 4];
    for i in &mut v {
        *i += 50; //dereferencing
        println!("{i}");
    }

    //Using enums to store multiple types in vectors
    enum Spreadsheet {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        Spreadsheet::Int(3),
        Spreadsheet::Float(5.6),
        Spreadsheet::Text(String::from("Ello"))
    ];

    //Dropping an element from vector
    //A vector is freed when it goes outta scope
    {
        let v = vec![1, 2, 3, 4];
    }  //Here v is dropped so its elements


}

