use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    // 乱数の生成
    let secret_number: i32 = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // mut 可変
    // String型の新しいインスタンスを生成
    let mut guess: String = String::new();

    io::stdin()
        // &mut guess は guessの参照
        // 入力内容をguessに格納するため、mutである必要がある
        .read_line(&mut guess)
        .expect("Failed to read line"); // エラーメッセージ

    println!("You guessed: {}", guess);
}
