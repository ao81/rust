use rand::Rng;
use std::io::{self, Write, stdout};

fn main() {
    let rand_num = rand::thread_rng().gen_range(1, 101);
	let mut cnt = 0;
    println!("1から100の数字を当ててください!");
    loop {
		cnt += 1;
		print!(">> ");
		stdout().flush().unwrap();
        let mut guess = String::new();
        let _ = io::stdin().read_line(&mut guess);
		let guess: i32 = match guess.trim().parse() {
			Ok(guess) => guess,
			Err(_) => {
				println!("有効な数字を入力してください");
				continue;
			}
		};
		if guess == rand_num {
			println!("正解です!");
			println!("{cnt} 回で正解しました!");
			break;
		} else if guess > rand_num {
			println!("{guess} よりも小さい数字です!");
		} else {
			println!("{guess} よりも大きい数字です!");
		}
    }
}
