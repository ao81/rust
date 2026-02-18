

fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // move
    let s3 = s2.clone(); // clone

    // println!("{s1}"); 所有権が移動済みのためエラー
    println!("{s2}");
    println!("{s3}");
}
