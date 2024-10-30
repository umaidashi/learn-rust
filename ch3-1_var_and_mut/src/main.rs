// should be error
//fn main() {
//    let x = 5;
//    println!("The value of x is: {}", x);     // xの値は{}です
//    x = 2;
//    println!("The value of x is: {}", x);
//}
//

//fn main() {
//    let mut x = 5;
//    println!("The value of x is: {}", x);
//    x = 6;
//    println!("The value of x is: {}", x);
//}
//result
//❯ cargo run
//   Compiling ch3-1_var_and_mut v0.1.0 (/Users/yu.oishi/Documents/git/learn-rust/ch3-1_var_and_mut)
//    Finished dev [unoptimized + debuginfo] target(s) in 0.76s
//     Running `target/debug/ch3-1_var_and_mut`
//The value of x is: 5
//The value of x is: 6

fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
