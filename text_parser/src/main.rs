pub fn mais_longa<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

pub fn conta_palavras(texto: &str) -> usize {
    texto.split_whitespace().count()
}

fn main() {
    let frase1 = String::from("Frase mais longa");
    let frase2 = String::from("Frase curta");
    let resultado = mais_longa(&frase1, &frase2);
    println!("Maior palavra: {}", resultado);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retorna_a_maior_palavra() {
        let a = "longaaaa";
        let b = "curta";
        assert_eq!(mais_longa(a, b), "longaaaa");
    }
}
