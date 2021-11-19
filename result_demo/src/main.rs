use std::{fs, io};
use std::io::Read;
use std::fs::File;
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("main.rs");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn read_usename_from_file_simple() -> Result<String, io::Error> {
    let mut f = File::open("main.rs")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_link() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("main.rs")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_api() -> Result<String, io::Error> {
    fs::read_to_string("main.rs")
}

fn main () {

}