// use std::io; // Importação da biblioteca para entrada de valores de textos

// Fundamentos de Rust

// Ponto de entrada do programa — primeira função executada pelo Rust
fn main() {
    // Comentário de uma linha
    /* Comentário de múltiplas linhas */
    // println! é uma macro (note o !) — exibe informações no terminal

    // Imprime texto
    // println!("Hello World!");

    // Para tipos primitivos, usa-se {} como placeholder dentro da string
    // e passa o valor depois da vírgula

    // Imprime inteiro
    // println!("{}", 3);

    // Imprime flutuante
    // println!("{}", 3.50);

    // Imprime booleano
    // println!("{}", true);

    /* ============================================================================================== */

    // Tipos de variáveis

    // Inteiros (números sem casas decimais)
    // let inteiro: i32 = 10;
    // let inteiro_negativo: i32 = -10;
    // let inteiro_positivo: u32 = 10; // u = unsigned (só positivo)

    // Flutuantes (números com casas decimais)
    // let flutuante: f64 = 3.14;

    // Booleano (verdadeiro ou falso)
    // let booleano: bool = true;

    // Texto fixo (valor definido no código)
    // let texto_fixo: &str = "olá";

    // Texto dinâmico (valor pode vir do usuário ou mudar)
    // let texto_dinamico: String = String::new();

    // Constante (nunca muda, tipo obrigatório)
    // const MINHA_CONSTANTE: u32 = 100;

    /* ============================================================================================== */

    // Operadores aritméticos

    // let a: i32 = 10;
    // let b: i32 = 3;

    // println!("{}", a + b);  // Adição        = 13
    // println!("{}", a - b);  // Subtração     = 7
    // println!("{}", a * b);  // Multiplicação = 30
    // println!("{}", a / b);  // Divisão       = 3 (inteiro, sem casas decimais)
    // println!("{}", a % b);  // Módulo        = 1 (resto da divisão)

    // Detalhe: divisão entre inteiros sempre retorna inteiro
    // let c: f64 = 10.0;
    // let d: f64 = 3.0;
    // println!("{}", c / d);  // 3.3333... (divisão com casas decimais)

    // Operadores de atribuição
    // let mut x: i32 = 10;
    // x += 5;  // x = x + 5 = 15
    // x -= 3;  // x = x - 3 = 12
    // x *= 2;  // x = x * 2 = 24
    // x /= 4;  // x = x / 4 = 6
    // x %= 4;  // x = x % 4 = 2

    /* ============================================================================================== */

    // Entrada de valores

    // println!("Digite seu nome: ");

    // let mut input = String::new(); // Declaração de uma variável mutável do tipo texto (String)
    // io::stdin().read_line(&mut input).expect("Erro"); // O módulo vai ler a entrada que o usuário digitou e caso não saia como previsto a função expect para tratamento de erros vai avisar a invalidação

    // Exibição do nome com tratamento
    // println!("Bem vindo ao mundo de rust {}", input.trim());

    /* ============================================================================================== */

    // Estruturas condicionais

    // if / else if / else
    // let numero = 14;
    // if numero < 12 {
    //     println!("Criança");
    // } else if numero < 18 {
    //     println!("Adolescente");
    // } else {
    //     println!("Adulto");
    // }

    // if como expressão (operador ternário do Rust)
    // let idade = 20;
    // let status = if idade >= 18 { "adulto" } else { "menor" };
    // println!("Você é {}", status);

    // match — equivalente ao switch/case
    // let dia = 1;
    // match dia {
    //     1 => println!("Segunda"),
    //     2 => println!("Terça"),
    //     3 => println!("Quarta"),
    //     4..=6 => println!("Quinta a Sábado"), // intervalo de valores
    //     7 => println!("Domingo"),
    //     _ => println!("Dia inválido"),        // _ é o "default", cobre o resto
    // }

    // match com múltiplas linhas por caso
    // let operador: &str = "+";
    // match operador {
    //     "+" => {
    //         println!("Operação: Adição");
    //         println!("Símbolo: +");
    //         println!("Exemplo: 2 + 2 = 4");
    //     }
    //     "-" => {
    //         println!("Operação: Subtração");
    //         println!("Símbolo: -");
    //     }
    //     "*" => println!("Multiplicação"),
    //     "/" => println!("Divisão"),
    //     _ => println!("Operador inválido"),
    // }

    /* ============================================================================================== */

    // Laços de repetições

    // loop — repete para sempre até o break
    // let mut contador = 0;
    // loop {
    //     contador += 1;
    //     println!("Contador: {}", contador);
    //     if contador == 5 {
    //         println!("Parou por aqui!");
    //         break;
    //     }
    // }

    // while — repete enquanto a condição for verdadeira
    // let mut contador = 0;
    // while contador < 5 {
    //     println!("Contador: {}", contador);
    //     contador += 1;
    // }

    // for — percorre um intervalo
    // for i in 1..5  { } // 1, 2, 3, 4 (não inclui o 5)
    // for i in 1..=5 { } // 1, 2, 3, 4, 5 (inclui o 5)
    // for i in 1..=5 {
    //     println!("Número: {}", i);
    // }

    // break e continue
    // for i in 1..=10 {
    //     if i % 2 == 0 {
    //         continue; // pula os pares
    //     }
    //     println!("{}", i); // imprime 1, 3, 5, 7, 9
    // }

    // Tabuada — combinando entrada de dados + for
    // println!("Digite o número para tabuada: ");
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("Erro");
    // let numero: i32 = input.trim().parse().expect("Digite um número válido!");
    // for i in 1..=10 {
    //     let tabuada: i32 = i * numero;
    //     println!("{} X {} = {}", numero, i, tabuada);
    // }

    /* ============================================================================================== */

    // Funções

    // Função sem parâmetro e sem retorno
    // hello_world_in_function();

    // Função com parâmetro e sem retorno
    // saudar("João Pedro");

    // Função com parâmetros e com retorno
    // let resultado = soma(3, 5);
    // println!("Resultado: {}", resultado);

    // Função com if como retorno (estilo idiomático Rust)
    // let resultado: i32 = maior(10, 20);
    // println!("O maior é {}", resultado);

    /* ============================================================================================== */

    // Listas, Vetores e tuplas

    // Vetores — métodos úteis
    // let mut numeros: Vec<i32> = vec![1, 2, 3, 4, 5]; // outra forma de criar já com valores
    // numeros.push(6);          // adiciona no final
    // numeros.pop();            // remove o último
    // println!("{}", numeros.len()); // tamanho do vetor
    // println!("{}", numeros[0]);    // acesso por índice

    // Percorrendo vetor com for
    // for numero in &numeros {
    //     println!("{}", numero);
    // }

    // Tuplas — desestruturação
    // let (nome, idade) = pessoa; // extrai os valores para variáveis
    // println!("Nome: {}, Idade: {}", nome, idade);

    // Arrays — percorrendo com for
    // let frutas = ["maçã", "banana", "uva"];
    // for fruta in frutas {
    //     println!("{}", fruta);
    // }

    /* ============================================================================================== */

    // Ownership, Borrowing e Lifetimes
    // — o trio que garante segurança de memória sem garbage collector

    // OWNERSHIP — cada valor tem um único dono
    // Regras:
    // 1. Cada valor tem um único dono (variável)
    // 2. Só pode ter um dono por vez
    // 3. Quando o dono sai de escopo, o valor é apagado da memória

    // Move — valor transferido, variável original não existe mais
    // let a = String::from("olá");
    // let b = a; // valor MOVIDO para b
    // println!("{}", a); // ERRO! a não existe mais
    // println!("{}", b); // funciona!

    // Copy — tipos simples são copiados, não movidos
    // let x = 5;
    // let y = x; // inteiros são COPIADOS
    // println!("{}", x); // funciona!
    // println!("{}", y); // funciona!

    // Move em funções — passar para função também move
    // fn imprimir(s: String) { println!("{}", s); }
    // let nome = String::from("João");
    // imprimir(nome);
    // println!("{}", nome); // ERRO! nome foi movido para a função

    /* ============================================================================================== */

    // BORROWING — emprestar sem transferir o dono

    // Referência imutável & — só leitura, várias ao mesmo tempo
    // fn imprimir(s: &String) { println!("{}", s); }
    // let nome = String::from("João");
    // imprimir(&nome); // empresta
    // println!("{}", nome); // funciona! nome ainda existe

    // Referência mutável &mut — pode modificar, só uma por vez
    // fn adicionar(s: &mut String) { s.push_str(" Silva"); }
    // let mut nome = String::from("João");
    // adicionar(&mut nome);
    // println!("{}", nome); // João Silva

    // Resumo das referências:
    // &nome      → imutável, várias simultâneas permitidas
    // &mut nome  → mutável, apenas uma simultânea permitida

    /* ============================================================================================== */

    // LIFETIMES — tempo de vida das referências
    // Garante que referências nunca apontem para memória já apagada

    // Problema que lifetimes resolve (dangling pointer):
    // let referencia;
    // {
    //     let valor = String::from("olá");
    //     referencia = &valor; // empresta valor
    // } // valor apagado aqui!
    // println!("{}", referencia); // ERRO! memória inválida

    // Na maioria dos casos Rust infere sozinho (lifetime elision)
    // fn saudar(nome: &str) -> &str { nome } // Rust entende o lifetime

    // Quando há múltiplas referências, você anota com 'a
    // fn maior<'a>(a: &'a str, b: &'a str) -> &'a str {
    //     if a.len() > b.len() { a } else { b }
    // } // 'a significa: "o retorno vive tanto quanto a e b"
}

// fn hello_world_in_function() {
//     println!("Hello World in a function!");
// }

// fn saudar(nome: &str) {
//     println!("Olá, {}!", nome);
// }

// fn soma(a: i32, b: i32) -> i32 {
//     a + b // retorno implícito
// }

// Retorno explícito com return
// fn maior(a: i32, b: i32) -> i32 {
//     if a > b {
//         return a;
//     } else {
//         return b;
//     }
// }

// Mesmo resultado com retorno implícito (estilo Rust)
// fn maior(a: i32, b: i32) -> i32 {
//     if a > b { a } else { b }
// }
