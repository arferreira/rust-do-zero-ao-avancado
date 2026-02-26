use minha_loja::Produto;
use minha_loja::produto::desconto::aplicar_desconto;
use minha_loja::produto::estoque::verificar_estoque;

fn main() {
    let produtos = vec![
        Produto {
            nome: String::from("Macbook"),
            preco: 1999.0,
        },
        Produto {
            nome: String::from("iPhone"),
            preco: 999.0,
        },
    ];

    if verificar_estoque(&produtos[0]) {
        let preco_final = aplicar_desconto(&produtos[0], 5.0);
        println!(
            "{}: de R$ {:.2} por R$ {:.2}",
            produtos[0].nome, produtos[0].preco, preco_final
        );
    }
}
