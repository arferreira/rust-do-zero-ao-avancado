pub mod desconto;
pub mod estoque;

pub struct Produto {
    pub nome: String,
    pub preco: f64,
}

pub fn calcular_total(produtos: &Vec<Produto>) -> f64 {
    produtos.iter().map(|p| p.preco).sum()
}
