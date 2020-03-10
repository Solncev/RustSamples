use std::{io, thread, time};
use rand::Rng;

fn main() {
    let chamber = rand::thread_rng().gen_range(1, 6);

    println!("Enter number of players:");

    let mut players = String::new();
    io::stdin().read_line(&mut players)
        .expect("Failed to read line");
    let n: i32 = players.trim().parse().unwrap();

    let mut flag = true;

    while flag {
        for i in 0..n {
            let x = rand::thread_rng().gen_range(1, 6);
            println!("Spinning revolver for player: {} revolver aligned to chamber {}", i + 1, x);
            thread::sleep(time::Duration::from_secs(1));
            println!("BOOM!");
            if x == chamber {
                println!("Player {} is dead", i + 1);
                flag = false;
                break;
            }
        }
    }
}
