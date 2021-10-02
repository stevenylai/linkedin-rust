use std::io;
use rand;

fn main() {
    let res = rand::random::<i32>() % 100 + 1;
    println!("Guess");
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);
        let guessed = buffer.trim().parse::<i32>();
        let output = match guessed {
            Ok(num) => {
                if num < res {
                    "higher"
                } else if num > res {
                    "lower"
                } else {
                    "correct"
                }
            }
            Err(_) => "invalid input"
        };
        println!("{}", output);
        if output == "correct" {
            break;
        }
    }
}
