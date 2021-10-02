use std::env;
use std::fs;

fn main() {
    if env::args().len() < 3 {
        println!("Usage: {} <filename> <name>", env::args().nth(0).unwrap());
        return;
    }
    let filename = env::args().nth(1).unwrap();
    let name = env::args().nth(2).unwrap();
    let file_contents = fs::read_to_string(filename);
    let res = match file_contents {
        Ok(v) => {
            let found = v.lines().filter(|p| p == &name).count();
            found
        }
        Err(_) => {
            print!("bad file. ");
            0
        }
    };
    if res > 0 {
        println!("found");
    } else {
        println!("not found");
    }
}
