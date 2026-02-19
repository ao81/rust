fn main() {
    let mut sl1 = String::from("Hello");    
    let sl2 = String::from("World");
    let len = calculate_length(&sl2);

    println!("The length of '{}' is {}.", sl2, len);

    change(&mut sl1, &sl2);
    println!("Changed sl1 is: {}", sl1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s1: &mut String, s2: &String) /* -> &String */ {
    s1.push_str(&s2);
    // s1
}