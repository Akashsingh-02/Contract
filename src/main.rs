// fn main() {
//     println!("Hello, world!");
// }
use rand ::{thread_rng, Rng};
use std::io;
use std::cmp::Ordering;

fn main(){
    println!("Welcome to Lottery game");
    let secret_no = rand::thread_rng().gen_range(1,101);

    loop{
        println!("enter the guessed no.(1-100)");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        println!("your guessed no.: {}", guess);

        let guess:u32 = guess.trim().parse().expect("Type an Integer");
        
        match guess.cmp(&secret_no){
            Ordering::Less => println!("guess a big no."),
            Ordering::Greater => println!("guess a small no."),
            Ordering::Equal => {
                println!("you Won!! hurray");
                break;
            },
        }
    }
}