// Aula: variáveis e constantes — ver GUIA_RUST.md (seções 2 a 5)
// Rode com: cargo run --example 01_variaveisConst

// ─── Globais: declaradas fora de qualquer função ────────────────────────────
// Tipo obrigatório e nome em SCREAMING_SNAKE_CASE.
const PI: f64 = 3.14159; // const: copiada em cada lugar onde é usada
const MAX_ALUNOS: usize = 40;

static TOTAL: i32 = 30; // static: um único endereço fixo na memória
static NOME_CURSO: &str = "Rust do Zero";

fn main() {
    // ─── 1. Variáveis (let) ─────────────────────────────────────────────────
    println!("--- 1. Variáveis ---");

    // Imutável por padrão; o compilador infere o tipo (aqui, i32).
    let idade = 25;
    println!("Idade: {}", idade);
    // idade = 26; // erro de compilação: não dá para reatribuir sem `mut`

    // Com `mut` o valor pode mudar.
    let mut contador = 0;
    contador += 1;
    contador += 1;
    println!("Contador: {}", contador);

    // Tipo explícito (anotação opcional em `let`).
    let altura: f32 = 1.75;
    let ativo: bool = true;
    let letra: char = 'R';
    let saudacao: &str = "Olá";
    println!("Altura: {}  Ativo: {}  Letra: {}  Saudação: {}", altura, ativo, letra, saudacao);

    // Tipo pelo sufixo do número; `_` só separa os milhares para leitura.
    let grande = 1_000_000u64;
    let pequeno = 200u8;
    println!("Grande: {}  Pequeno: {}", grande, pequeno);

    // Declarar agora e atribuir uma única vez depois (continua imutável).
    let resultado: i32;
    if grande > 1_000 {
        resultado = 1;
    } else {
        resultado = 0;
    }
    println!("Resultado: {}", resultado);

    // Prefixo `_` avisa ao compilador que não usar a variável é intencional.
    let _nao_usada = 99;

    // ─── 2. Shadowing ───────────────────────────────────────────────────────
    println!("\n--- 2. Shadowing ---");

    // Um novo `let` com o mesmo nome cria outra variável — pode até mudar o tipo.
    let valor = "42"; // &str
    let valor: i32 = valor.parse().unwrap(); // i32
    let valor = valor * 2;
    println!("Valor: {}", valor);

    // ─── 3. Constantes e estáticos ──────────────────────────────────────────
    println!("\n--- 3. Constantes e estáticos ---");

    // Globais (declaradas no topo do arquivo).
    println!("Curso: {}  |  Total: {}", NOME_CURSO, TOTAL);
    println!("PI: {}  |  Máximo de alunos: {}", PI, MAX_ALUNOS);

    // const local: só existe dentro desta função.
    const DESCONTO: f64 = 0.10;
    let preco = 100.0;
    let preco_final = preco * (1.0 - DESCONTO);
    println!("Preço final: {}", preco_final);

    // let × const
    let nome = "code"; // let: tipo inferido, snake_case
    const MEU_PASSARALHO: &str = "xibata"; // const: tipo obrigatório, SCREAMING_SNAKE_CASE
    println!("Nome: {}", nome);

    // A const é copiada em cada uso: as duas linhas geram o mesmo código no binário.
    println!("Nome da Empresa: {}", MEU_PASSARALHO);
    println!("Nome da Empresa: {}", "xibata");

    // ─── 4. Escopo ──────────────────────────────────────────────────────────
    println!("\n--- 4. Escopo ---");

    // Variável criada dentro de { } some ao fechar o bloco.
    {
        let interna = "só existe aqui dentro";
        println!("{}", interna);
    }
    // println!("{}", interna); // erro: `interna` não existe aqui fora

    // Um bloco também é uma expressão: a última linha (sem `;`) vira o valor.
    let dobro = {
        let base = 21;
        base * 2
    };
    println!("Dobro: {}", dobro);
}
