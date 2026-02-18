fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = 10;
    let y = y + 1;
    {
        let y = y * 2;
        println!("y = {y}");
    }
    println!("y = {y}");

    let spaces = "   ";
    let a;
    a = spaces.len();

    println!("\"{spaces}\" size is: {a}");
}
