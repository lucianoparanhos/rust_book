use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Advinhe o número");
    let secret_value = rand::thread_rng().gen_range(1..101);
    println!("O número secreto: {}", secret_value);

    loop {
        println!("Digite um número: ");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Erro ao ler");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Seu palpite: {}", guess);

        match guess.cmp(&secret_value) {
            Ordering::Less => println!("Muito pequeno!"),
            Ordering::Greater => println!("Muito grande!"),
            Ordering::Equal => {
                println!("Você ganhou");
                break;
            }
        }
    }
}
