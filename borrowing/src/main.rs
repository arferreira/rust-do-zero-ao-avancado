fn main() {
    let s = String::from("Antonio esta ensinando Rust");
    let s2 = s.clone();
    println!("valor de s e: {s}");

    let name = String::from("Antonio"); // 0x8uiej2iefwkdnp
    imprime_nome(&name); // &String
    println!("nome: {name}");

    let mut antonio = String::from("Antonio");
    let z = &antonio;
    println!("z: {z}");
    let antonio = altera_nome(&mut antonio);
    println!("meu nome e: {antonio}");

    let a = 10;
    let b = a;
    println!("valor de a: {a}");
}

// fn gera_string() -> &String {
//     let s = String::from("Eu estou aprendendo Rust");
//     &s
// }

fn altera_nome(nome: &mut String) -> &String {
    nome.push_str(" Souza");
    nome
}

fn imprime_nome(nome: &String) {
    println!("meu nome e: {nome}")
}
