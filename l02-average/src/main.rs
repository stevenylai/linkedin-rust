fn main() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = ((a as f64) + (b as f64) + (c as f64)) / 3.0f64;
    /* YOUR CODE GOES HERE */

    assert_eq!(average, 45.1);
    println!("Test passed!");
}
