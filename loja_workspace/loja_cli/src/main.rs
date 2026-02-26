use loja_core::Produto;
use loja_core::calcular_total;

fn main() {
    let produtos = vec![
        Produto {
            nome: String::from("Macbook"),
            preco: 899.0,
        },
        Produto {
            nome: String::from("iPhone"),
            preco: 399.0,
        },
    ];

    let total = calcular_total(&produtos);
    println!("Total: R$ {:.2}", total);
}
