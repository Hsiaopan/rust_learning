use std::fs::File;
use std::io::{Error as IoError, Read};
use std::path::PathBuf;

fn read_file_contents(path: PathBuf) -> Result<String, IoError>{
    let mut string = String::new();

    // 1. Handle this match expression
    // ------------------------------------------------------
    // pass the variable to the 'file' variable on success, or
    // Return from the function early if it is an error
    let mut file: File = match File::open(path) {
        Ok(file_handle) => file_handle,
        Err(io_error) => return Err(io_error),
    }; 

    // 2. Handle this error
    // ------------------------------------------------------
    // The success path is already filled in for you
    // Return from the function early if it is an error
    match file.read_to_string(&mut string){
        Ok(_) => (),
        Err(io_error) => return Err(io_error),
    };

    // 3.Return the 'String' variable as expected by this function signature
    Ok(string)
}

fn main() {
    assert!(read_file_contents(PathBuf::from("src/main.rs")).is_ok());
    assert!(read_file_contents(PathBuf::from("no-existent-file.txt")).is_err());
}
