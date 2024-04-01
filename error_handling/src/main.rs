use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn main() {
    //Panic is unwanted situation in code
    //By default when panic occurs - program starts unwinding, walks back up stack and clear data of functions
    //Or you can choose to abort immediately without unwinding (OS will need to clean up mem then)
    // panic!("crash and burn!");

    let v = vec![1, 2, 3];

    // v[99]; //program panics to avoid buffer_overread (a sec vul)

    //backtrace is a list of all the functions that have been called to get to this point $RUST_BACKTRACE=1 cargo run

    //Handling recoverable errors with Result
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") { 
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", error)

            },
            other_error => panic!("Problem opening the file: {:?}", error),
        }   
    };

    //Shortcuts for panic: unwrap and expect
    // let greeting_file_result = File::open("hello.txt").unwrap(); //unwrap method is a shortcut method implemented just like the match expression

    // let greeting_file_result = File::open("hello.txt").expect("This file should pe present inside project"); //the expect method lets us also choose the panic! error message - a far better way


    match read_username_from_file() {
        Ok(username) => println!("username read from file is: {username}"),
        Err(e) => panic!("Error: {e} reading username")
    };

}

//Propagating errors 
fn read_username_from_file() -> Result<String, io::Error> {
    let file = File::open("hello.txt");

    let mut username_file = match file {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

