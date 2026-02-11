fn main() {
    let s = String::from("Antonio esta ensinando Rust");
    let s2 = s.clone();
    println!("valor de s e: {s}");

    let name = String::from("Antonio");
    imprime_nome(name.clone());
    println!("nome: {name}");

    let antonio = String::from("Antonio");
    let antonio = altera_nome(antonio);
    println!("meu nome e: {antonio}");

    let a = 10;
    let b = a;
    println!("valor de a: {a}");
}

fn altera_nome(mut nome: String) -> String {
    nome.push_str(" Souza");
    nome
}

fn imprime_nome(nome: String) {
    println!("meu nome e: {nome}")
}
