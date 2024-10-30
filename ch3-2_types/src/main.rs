fn main() {
    //let guess = "42".parse().expect("Not a number!");
    //type annotations needed

    let guess: u32 = "42".parse().expect("Not a number!");
    print!("The value of guess is: {}", guess);
    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {}", quotient);
    let floored = 2 / 3; // Results in 0
    println!("The value of floored is: {}", floored);

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {}", remainder);

    // boolean
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("The value of t is: {}", t);
    println!("The value of f is: {}", f);

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x); // tap.0
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    //array
    let arr = [1, 2, 3, 4];
    println!("The value of arr is: {}", arr[0]);
}
