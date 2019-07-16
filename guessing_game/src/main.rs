use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("guessing game");
    let randnum=rand::thread_rng().gen_range(1,101);
    loop{
        println!("please input a number");
        let mut guess=String::new();
        io::stdin().read_line(&mut guess)
                    .expect("Failed to read line");
        let guess: u32=match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue
        };
        println!("your guess:{}",guess);
        match guess.cmp(&randnum){
            Ordering::Less=>println!("too small"),
            Ordering::Greater=>println!("too big"),
            Ordering::Equal=>{
                println!("you win");
                break;
            }
        }
    }
}
