use std::fmt;
use std::fmt::Formatter;

struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

impl std::fmt::Display for Satellite {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.name, self.velocity)
    }
}
/* YOUR CODE GOES HERE */

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    println!("hubble is {}", hubble);
}
