use rand::Rng; //trait
use std::cmp::Ordering;
use std::io; // prelude

fn main() {
    println!("猜数");
    let secret_number = rand::thread_rng().gen_range(0..100);
    println!("神秘数字是: {}", secret_number);
    
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect(" 无法读取行");

        // shadow
        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("你猜测的数是: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("you win");
                break;}
                ,
        }
    }
}
