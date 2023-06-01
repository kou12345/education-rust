fn main() {
    println!("Hello, world!");

    another_function();
    function_with_parameter(5);
    print_labeled_measurement(10, "m");

    let y = {
        let x = 3;
        x + 1 // return x + 1; と同じかな
        // 式の終端にセミコロンを付けたら、文に変えてしまいます。
    };

    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}

fn function_with_parameter(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, label: &str) {
    println!("The measurement is: {}{}", value, label);
}

// 関数本体は、文が並び、最後に式を置くか文を置くという形で形成される
// Rustは式指向言語

// 関数の戻り値は、関数本体の最後の式の値になる
// return キーワードを使って、早期に関数から戻ることもできる
fn five() -> i32 {
    5
}