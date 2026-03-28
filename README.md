# 🦀 Fundamentos de Rust

Repositório de estudos da linguagem Rust, cobrindo os conceitos fundamentais da linguagem.

## ⚙️ Requisitos e Instalação

### Windows
Antes de instalar o Rust, é necessário ter o **Visual Studio C++ Build Tools**:
1. Baixe em [visualstudio.microsoft.com](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
2. Instale o componente **"Desenvolvimento para desktop com C++"**

Depois instale o Rust pelo instalador oficial:
1. Baixe o `rustup-init.exe` em [rustup.rs](https://rustup.rs)
2. Execute e siga as instruções

### Linux / macOS
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Verificando a instalação
```bash
rustc --version   # verifica o compilador
cargo --version   # verifica o gerenciador de pacotes
```

### O que é instalado
| Ferramenta | O que faz |
|---|---|
| `rustc` | Compilador do Rust |
| `cargo` | Gerenciador de projetos e pacotes |
| `rustup` | Gerenciador de versões do Rust |

### Extensão recomendada
- **VS Code** + extensão **rust-analyzer** — autocomplete e erros em tempo real

## 📁 Estrutura
```
├── 📁 hello_rust        # Primeiro projeto — Hello World e conceitos básicos
├── 📁 project1          # Conversor de moedas
├── 📁 project2          # Calculadora simples
├── 📁 project3          # Jogo de adivinhação
└── 📝 README.md
```

## 📚 Conceitos Aprendidos

### Fundamentos
- Saída de dados com `println!`
- Tipos primitivos: `i32`, `u32`, `f64`, `bool`, `&str`, `String`
- Variáveis imutáveis `let` e mutáveis `let mut`
- Constantes `const`
- Operadores aritméticos e de atribuição

### Entrada de Dados
- Leitura do terminal com `std::io`
- Conversão de tipos com `.parse()`
- Tratamento de erros com `.expect()` e `match Ok/Err`

### Estruturas de Controle
- Condicionais: `if`, `else if`, `else`
- `if` como expressão (operador ternário)
- `match` — equivalente ao switch/case

### Laços de Repetição
- `loop` — repetição infinita com `break`
- `while` — repetição condicional
- `for` — iteração sobre intervalos e coleções
- `break` e `continue`

### Funções
- Funções sem retorno
- Funções com parâmetros e retorno
- Retorno implícito (sem `;`) e explícito (`return`)

### Coleções
- Tuplas — tipos diferentes, tamanho fixo
- Arrays — mesmo tipo, tamanho fixo
- Vetores `Vec<T>` — mesmo tipo, tamanho dinâmico

### Gerenciamento de Memória
- **Ownership** — cada valor tem um único dono
- **Borrowing** — empréstimo com `&` e `&mut`
- **Lifetimes** — tempo de vida das referências

## 🛠️ Projetos

| Projeto | Descrição | Conceitos praticados |
|---|---|---|
| `hello_rust` | Hello World e fundamentos | Variáveis, tipos, macros |
| `project1` | Conversor de moedas | Funções, match, loop, entrada de dados |
| `project2` | Calculadora simples | Tratamento de erros, Ok/Err, loops |
| `project3` | Jogo de adivinhação | Crate externo `rand`, loop duplo |

## 🚀 Como executar
```bash
cd nome_do_projeto
cargo run
```

## 📖 Recursos utilizados

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)