use rand::Rng;
use std::io;

fn main() {
    println!("Jogo da adivinhação!");
    println!("============================================");

    loop { 
        let secret_number: u32 = rand::rng().random_range(1..=100);
        println!("Tente adivinhar o número secreto entre 1 e 100.");

        loop { 
            println!("Digite seu palpite:");

            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Falha ao ler a entrada.");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Por favor, digite um número válido.");
                    continue;
                }
            };

            if guess < secret_number {
                println!("Muito baixo! Tente novamente.");
            } else if guess > secret_number {
                println!("Muito alto! Tente novamente.");
            } else {
                println!("Parabéns! Você adivinhou o número secreto!");

                println!("Deseja jogar novamente? (s/n)");
                let mut play_again = String::new();
                io::stdin()
                    .read_line(&mut play_again)
                    .expect("Falha ao ler a entrada.");

                if play_again.trim().to_lowercase() == "s" {
                    break; 
                } else {
                    println!("Obrigado por jogar!");
                    return;
                }
            }
        }
    }
}