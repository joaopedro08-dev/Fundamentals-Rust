fn main() {
    loop {
        println!("Calculadora simples");
        println!("================================");
        println!("Selecione a operação:");
        println!("0. Sair");
        println!("1. Adição");
        println!("2. Subtração");
        println!("3. Multiplicação");
        println!("4. Divisão");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a entrada");
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida. Por favor, tente novamente.");
                continue;
            }
        };

        match choice {
            0 => {
                sair();
                break;
            }
            1 => adicao(),
            2 => subtracao(),
            3 => multiplicacao(),
            4 => divisao(),
            _ => println!("Opção inválida. Por favor, tente novamente."),
        }
    }
}

fn adicao() {
    loop {
        println!("Digite o primeiro número:");
        let mut num1 = String::new();
        std::io::stdin()
            .read_line(&mut num1)
            .expect("Falha ao ler a entrada");
        let num1: f64 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida. Por favor, tente novamente.");
                continue;
            }
        };

        println!("Digite o segundo número:");
        let mut num2 = String::new();
        std::io::stdin()
            .read_line(&mut num2)
            .expect("Falha ao ler a entrada");
        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida. Por favor, tente novamente.");
                continue;
            }
        };

        let resultado = num1 + num2;
        println!("O resultado da adição é: {}", resultado);

        println!("Deseja continuar a operação? (s/n)");
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Falha ao ler a entrada");

        if choice.trim().to_lowercase() != "s" {
            break;
        }
    }
}

fn subtracao() {
    loop {
        println!("Digite o primeiro número:");
        let mut num1 = String::new();
        std::io::stdin()
            .read_line(&mut num1)
            .expect("Falha ao ler a entrada");
        let num1: f64 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida. Por favor, tente novamente.");
                continue;
            }
        };

        println!("Digite o segundo número:");
        let mut num2 = String::new();
        std::io::stdin()
            .read_line(&mut num2)
            .expect("Falha ao ler a entrada");
        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida. Por favor, tente novamente.");
                continue;
            }
        };

        let resultado = num1 - num2;
        println!("O resultado da subtração é: {}", resultado);

        println!("Deseja continuar a operação? (s/n)");
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Falha ao ler a entrada");

        if choice.trim().to_lowercase() != "s" {
            break;
        }
    }
}

fn multiplicacao() {
    loop {
        println!("Digite o primeiro número:");
        let mut num1 = String::new();
        std::io::stdin()
            .read_line(&mut num1)
            .expect("Falha ao ler a entrada");
        let num1: f64 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida. Por favor, tente novamente.");
                continue;
            }
        };

        println!("Digite o segundo número:");
        let mut num2 = String::new();
        std::io::stdin()
            .read_line(&mut num2)
            .expect("Falha ao ler a entrada");
        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida. Por favor, tente novamente.");
                continue;
            }
        };

        let resultado = num1 * num2;
        println!("O resultado da multiplicação é: {}", resultado);

        println!("Deseja continuar a operação? (s/n)");
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Falha ao ler a entrada");

        if choice.trim().to_lowercase() != "s" {
            break;
        }
    }
}

fn divisao() {
    loop {
        println!("Digite o primeiro número:");
        let mut num1 = String::new();
        std::io::stdin()
            .read_line(&mut num1)
            .expect("Falha ao ler a entrada");
        let num1: f64 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida. Por favor, tente novamente.");
                continue;
            }
        };

        println!("Digite o segundo número:");
        let mut num2 = String::new();
        std::io::stdin()
            .read_line(&mut num2)
            .expect("Falha ao ler a entrada");
        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida. Por favor, tente novamente.");
                continue;
            }
        };

        if num2 == 0.0 {
            println!("Divisão por zero não é permitida. Por favor, tente novamente.");
            continue;
        }

        let resultado = num1 / num2;
        println!("O resultado da divisão é: {}", resultado);

        println!("Deseja continuar a operação? (s/n)");
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Falha ao ler a entrada");

        if choice.trim().to_lowercase() != "s" {
            break;
        }
    }
}

fn sair() {
    println!("Encerrando o programa.");
}