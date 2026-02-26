use super::Produto;

pub fn aplicar_desconto(produto: &Produto, percentual: f64) -> f64 {
    let desconto = produto.preco * (percentual / 100.0);
    produto.preco - desconto
}
