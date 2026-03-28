use std::io;

fn main() {
    loop {
        println!("Conversor de moedas");
        println!("============================================================");
        println!("Selecione a moeda de origem:");
        println!("0. Sair");
        println!("1. Dólar (USD)");
        println!("2. Euro (EUR)");
        println!("3. Real (BRL)");

        let mut origem = String::new();
        std::io::stdin()
            .read_line(&mut origem)
            .expect("Falha ao ler a entrada");

        println!("Selecione a moeda de destino:");
        println!("0. Sair");
        println!("1. Dólar (USD)");
        println!("2. Euro (EUR)");
        println!("3. Real (BRL)");

        let mut destino = String::new();
        std::io::stdin()
            .read_line(&mut destino)
            .expect("Falha ao ler a entrada");

        match (origem.trim(), destino.trim()) {
            ("0", _) | (_, "0") => {
                sair();
                break;
            }
            ("1", "2") => converter_dolar_para_euro(),
            ("1", "3") => converter_dolar_para_real(),
            ("2", "1") => converter_euro_para_dolar(),
            ("2", "3") => converter_euro_para_real(),
            ("3", "1") => converter_real_para_dolar(),
            ("3", "2") => converter_real_para_euro(),
            _ => println!("Opção inválida, por favor tente novamente."),
        }

        println!("Deseja realizar outra conversão? (s/n)");
        let mut resposta = String::new();
        std::io::stdin()
            .read_line(&mut resposta)
            .expect("Falha ao ler a entrada");
        if resposta.trim().to_lowercase() != "s" {
            sair();
            break;
        }
    }
}

fn converter_dolar_para_euro() {
    println!("Digite o valor em Dólar (USD):");
    let mut valor = String::new();
    std::io::stdin()
        .read_line(&mut valor)
        .expect("Falha ao ler a entrada");
    let valor: f64 = valor.trim().parse().expect("Por favor, insira um número válido");
    let resultado = valor * 0.85;
    println!("{} Dólar (USD) é igual a {:.2} Euro (EUR)", valor, resultado);
}

fn converter_dolar_para_real() {
    println!("Digite o valor em Dólar (USD):");
    let mut valor = String::new();
    std::io::stdin()
        .read_line(&mut valor)
        .expect("Falha ao ler a entrada");
    let valor: f64 = valor.trim().parse().expect("Por favor, insira um número válido");
    let resultado = valor * 5.25; 
    println!("{} Dólar (USD) é igual a {:.2} Real (BRL)", valor, resultado);
}

fn converter_euro_para_dolar() {
    println!("Digite o valor em Euro (EUR):");
    let mut valor = String::new();
    std::io::stdin()
        .read_line(&mut valor)
        .expect("Falha ao ler a entrada");
    let valor: f64 = valor.trim().parse().expect("Por favor, insira um número válido");
    let resultado = valor * 1.18; 
    println!("{} Euro (EUR) é igual a {:.2} Dólar (USD)", valor, resultado);
}

fn converter_euro_para_real() {
    println!("Digite o valor em Euro (EUR):");
    let mut valor = String::new();
    std::io::stdin()
        .read_line(&mut valor)
        .expect("Falha ao ler a entrada");
    let valor: f64 = valor.trim().parse().expect("Por favor, insira um número válido");
    let resultado = valor * 6.15; 
    println!("{} Euro (EUR) é igual a {:.2} Real (BRL)", valor, resultado);
}

fn converter_real_para_dolar() {
    println!("Digite o valor em Real (BRL):");
    let mut valor = String::new();
    std::io::stdin()
        .read_line(&mut valor)
        .expect("Falha ao ler a entrada");
    let valor: f64 = valor.trim().parse().expect("Por favor, insira um número válido");
    let resultado = valor * 0.19; 
    println!("{} Real (BRL) é igual a {:.2} Dólar (USD)", valor, resultado);
}

fn converter_real_para_euro() {
    println!("Digite o valor em Real (BRL):");
    let mut valor = String::new();
    std::io::stdin()
        .read_line(&mut valor)
        .expect("Falha ao ler a entrada");
    let valor: f64 = valor.trim().parse().expect("Por favor, insira um número válido");
    let resultado = valor * 0.16; 
    println!("{} Real (BRL) é igual a {:.2} Euro (EUR)", valor, resultado);
}

fn sair() {
    println!("Encerrando o programa.");
}