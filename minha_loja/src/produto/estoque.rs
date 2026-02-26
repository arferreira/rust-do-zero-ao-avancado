use super::Produto;

pub fn verificar_estoque(produto: &Produto) -> bool {
    // simular o estoque
    println!("Vericando o estoque do produto: {}...", produto.nome);
    true
}
