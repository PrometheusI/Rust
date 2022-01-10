use std::fs::{self,File};
use std::io::{ErrorKind, Error, Read};
use std::error::Error as Error2;

fn main() -> Result<(), Box<dyn Error2>> {
    let _f = File::open("hello.txt")?;
   println!("{:?}",read_username_from_file_shorter());
   Ok(())
}

fn _unrecoverable_error(){
    //panic!("Burn and crash");
    let vec = vec![1,2,3];

    vec[99];
}

fn _recoverable_error(){
    let f = File::open("hello.txt");

    let _f = match f{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fs) => fs,
                Err(e) => panic!("File could not be created: {:?}",e)
            },
            other_error => panic!("Problem with opening the file: {:?}",other_error)
        }
    };

    
}
fn _recoverable_error_clean(){
    let _f = File::open("hello.txt");

    //cleaner code
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn _unwrap_file(){
    let _f = File::open("hello.txt").unwrap();
}

fn _expect_file(){
    let _f = File::open("hello.txt").expect("Failed to open the file.");
}

fn _read_username_from_file() -> Result<String, Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn _read_username_from_file_short() -> Result<String, Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_shorter() -> Result<String, Error>{
    fs::read_to_string("hello.txt")
}