use macroquad::prelude::*;

const SIZE: f32 = 15.0;

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[macroquad::main("Snake Game")]
async fn main() {
    let mut x = 10;
    let mut y = 10;
    let mut dir = Dir::Right;
    let mut last_update = get_time();

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::W) { dir = Dir::Up; }
        if is_key_pressed(KeyCode::S) { dir = Dir::Down; }
        if is_key_pressed(KeyCode::A) { dir = Dir::Left; }
        if is_key_pressed(KeyCode::D) { dir = Dir::Right; }

        if get_time() - last_update > 0.1 {
            match dir {
                Dir::Up		=> y -= 1,
                Dir::Down	=> y += 1,
                Dir::Left	=> x -= 1,
                Dir::Right	=> x += 1,
            }
            last_update = get_time();
        }

        draw_rectangle(
            x as f32 * SIZE,
            y as f32 * SIZE,
            SIZE - 1.0,
            SIZE - 1.0,
            GREEN,
        );

        next_frame().await;
    }
}
