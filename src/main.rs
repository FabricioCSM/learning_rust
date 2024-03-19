use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adivinhe um número");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Por favor insira um número");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("Você adivinhou: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Abaixo"),
            Ordering::Greater => println!("Acima"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
    }
}