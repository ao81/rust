fn main() {
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    let f: bool = false;
    if !f {
        println!("Hello");
    }

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of tup is: {x}, {y}, {z}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];

    println!("a[3] = {}", a[3]);
    println!("b[2] = {}", b[2]);
}
