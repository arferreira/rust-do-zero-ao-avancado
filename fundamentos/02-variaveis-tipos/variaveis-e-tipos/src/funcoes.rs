// Funções
// + , - , *, /
// Crie uma função de tabuada

fn soma(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn soma_dobrado(mut n1: i32, mut n2: i32) -> i32 {
    n1 = n1 * 2;
    n2 = n2 * 2;
    n1 + n2
}

fn bom_dia(name: &str, _sobrenome: &str) {
    println!("Olá {name}, bom dia!");
}

pub fn principal() {
    let name: &str = "Antonio";
    bom_dia(name, "Souza");

    // soma
    let n1 = 20;
    let n2 = 30;
    let resultado = soma(n1, n2);
    println!("O resultado da soma é: {resultado}");

    // soma dobrado
    let resultado = soma_dobrado(n1, n2);
    println!("O resultado da soma é: {resultado}");
}
