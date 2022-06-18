use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct GuessNumber {}

impl GuessNumber {
    pub fn guess() {
        let target = rand::thread_rng().gen_range(1..101);
        println!("数字范围: [1, 100]");

        loop {
            let mut my_guess = String::new();
            io::stdin().read_line(&mut my_guess).expect("读取出错");
            let my_guess: u32 = match my_guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("invalid input, retry!");
                    continue;
                }
            };

            match my_guess.cmp(&target) {
                Ordering::Less => println!("too small!"),
                Ordering::Greater => println!("too big!"),
                Ordering::Equal => {
                    println!("that is!");
                    break;
                }
            }
        }
    }
}