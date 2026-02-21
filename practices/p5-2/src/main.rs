struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );

    let rect1 = Rect {
        width: 20,
        height: 40,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area2(&rect1)
    )
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(rect: &Rect) -> u32 {
    rect.width * rect.height
}
