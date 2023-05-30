use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    // 乱数の生成
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // mut 可変
        // String型の新しいインスタンスを生成
        let mut guess: String = String::new();

        io::stdin()
            // &mut guess は guessの参照
            // 入力内容をguessに格納するため、mutである必要がある
            .read_line(&mut guess)
            .expect("Failed to read line"); // エラーメッセージ

        // Stringインスタンスのtrimメソッドは文字列の先頭と末尾の空白をすべて削除する
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // 予想と秘密の数を比較
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
