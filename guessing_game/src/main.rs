use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // mut 可変
    // String型の新しいインスタンスを生成
    let mut guess = String::new();

    io::stdin()
        // &mut guess は guessの参照
        // 入力内容をguessに格納するため、mutである必要がある
        .read_line(&mut guess)
        .expect("Failed to read line"); // エラーメッセージ

    println!("You guessed: {}", guess);
}
