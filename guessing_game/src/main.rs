use rand::Rng;
use std::cmp::Ordering;
use std::io; // 標準ライブラリの import

fn main() {
    println!("Guess the number!"); // 数を当ててごらん

    let secret_number = rand::thread_rng().gen_range(1..101); // 1 から 100 までの乱数を生成

    println!("Please input your guess."); // ほら、予想を入力してね

    let mut guess = String::new(); // 可変変数を宣言し、空の文字列を割り当てる1
                                   // Rust ではデフォルトの変数は不変
                                   // :: 構文は関連関数の呼び出し

    io::stdin() // 標準入力を待つ std::io::stdin() でも可
        .read_line(&mut guess) // 参照を渡して、入力を受け取る.
        .expect("Failed to read line"); // Result型のエラーチェック、これが無い場合はWarning
                                        // 参照もデフォルトで不変なので、&mut で可変にする

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess); // 次のように予想しました: {}

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    };
}
