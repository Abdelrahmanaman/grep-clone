use std::fs::File;
use std::io::{self, Read};
fn main(){
    
    match read_file_to_string("foo.txt") {
        Ok(s) => print!("{}", s),
        Err(e) => print!("Error: {}", e)
    }


}

fn read_file_to_string(file_path: &str) -> Result<String, io::Error> {

    let mut file = File::open(file_path)?;
    let mut file_content= String::new();

    file.read_to_string(&mut file_content)?;

    Ok(file_content)

}