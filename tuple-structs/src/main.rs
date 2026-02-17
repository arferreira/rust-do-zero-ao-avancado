struct Largura(i32);
struct Altura(i32);

fn cria_retangulo(largura: Largura, altura: Altura) {
    println!("{}x{}", largura.0, altura.0);
}

fn main() {
    cria_retangulo(Largura(200), Altura(100));
}
