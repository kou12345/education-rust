// タプル型
pub fn tuple_type() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    println!("The value of x is: {}", tup.0);
    println!("The value of z is: {}", tup.2);
}

// 配列型
// Rustの配列は、固定長。
// 一度宣言されたら、サイズを変えることはできない
pub fn array_type() {
    let a = [1, 2, 3, 4, 5];
    // 配列aの中身を表示
    println!("The value of a is: {:?}", a);

    let b = [3; 5];
    // b = [3, 3, 3, 3, 3] と同じ
    
}