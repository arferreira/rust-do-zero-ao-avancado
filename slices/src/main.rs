fn main() {
    // Slice a parte sequencia de uma colecao
    let frase: String =
        String::from("Essa e uma string gigantesca declarada com o tipo String em Rust");
    let primeira_parte: &str = primeira_parte(&frase);
    println!("{primeira_parte}");
}

fn primeira_parte(frase: &str) -> &str {
    &frase[..15]
}
