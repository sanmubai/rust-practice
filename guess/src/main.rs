use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("guess!");
    let rand=rand::thread_rng().gen_range(1,101);

    loop {
        println!("请输入一个数：");
        let mut guess_string=String::new();
        io::stdin().read_line(&mut guess_string).expect("error");

        let guess_string:i32=match guess_string.trim().parse() {
            Ok(num)=>num,
            Err(_)=>continue
        };
        println!("read line string {}",guess_string);
        match guess_string.cmp(&rand) {
            Ordering::Less=>println!("太小"),
            Ordering::Greater=>println!("太大"),
            Ordering::Equal=>{
                println!("你赢了");
                break;
            }
        }
    }

}
