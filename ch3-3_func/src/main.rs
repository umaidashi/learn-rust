fn main() {
    println!("Hello, world!");

    snake_case();
    print_arg(42);
    print_args(42, 'h');
    statement_expression();

    let x = has_return();
    println!("x: {}", x);
}

fn snake_case() {
    println!("snake_case called!");
}

fn print_arg(num: i32) {
    println!("num is {}", num);
}

fn print_args(num: i32, c: char) {
    println!("{} {}", num, c);
}

// statement, expression
fn statement_expression() {
    let y = 6; // this is statement, 6 is expression
    println!("{}", y);

    let z = {
        let h = 5;
        h + 4 // block is expression (not has ';' in the end)
    };
    println!("{}", z);
}

fn has_return() -> i32 {
    //return 42;
    42

    // return xx; or xx
}
