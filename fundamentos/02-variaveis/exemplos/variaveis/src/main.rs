/*
*
* Variaveis
*
* Exercícios
* - Exe1 : https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=d0dde26ba6aba374f932059ed58f9259
* - Exe2: https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=941789d869d16dee173a6ebf4c49f12a
* - Exe3: https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=0abcbb7f268026f5fa057ed2b730e899
* */

fn main() {
    let mut x: i32 = 10; // signed 
    let y: u32 = 8; // unsigned

    let z: f64 = 3.14333;

    let name: String = String::from("antonio");
    let _name2: &str = "antonio";

    println!("x é: {x}, y é: {y} e z é: {z}");

    // mutabilidade
    x = 11;
    println!("x é: {x}");

    println!("nome é: {name}");

    {
        // escopo que muda o nome para joao
        let name = "joão";
        let base_calculo = 2;
        println!("nome é: {name}");
        x = 1000 * base_calculo;
    }

    // ira quebrar pois foi definido apenas no escopo acima
    // println!("base calculo é: {}", base_calculo);

    println!("x é: {x}");
    // escopo da funcao main
    println!("nome é: {name}");

    // shadowing
    let _idade = 37;
    // idade = 16; nao conseguirei mudar o valor em razao da mutabilidade
    let idade = 16;

    println!("idade é: {idade}");

    // constantes
    const TENTATIVAS: i8 = 10;
    println!("TENTATIVAS é: {TENTATIVAS}");

    //char
    let primeira_letra: char = 'a';
    println!("primeira letra é: {primeira_letra}");
}

