fn another_function() {
    println!("Another function.");
}

fn sum(n: i32) -> i32 {
    // 関数の最後に数字を置くことで戻り値となる
    if n == 1 {
        1
    } else {
        n + sum(n - 1)
    }
}

fn increment(n: i32) -> i32 {
    n + 1
}

fn main() {
    println!("Hello, world!");

    another_function();

    let sum = sum(10);

    println!("1~10 sum: {}", sum);

    println!("increment: {}", increment(sum));
}
