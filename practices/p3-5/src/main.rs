fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    let mut counter = 0;
    loop {
        counter += 1;
        print!("#");
        if counter == 10 {
            break;
        }
    }
    println!();

    counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let nums = [1, 2, 3, 4, 5];
    for num in nums {
        print!("{}", num);
    }
    println!();

    for num in (1..5).rev() {
        println!("{num}");
    }
}
