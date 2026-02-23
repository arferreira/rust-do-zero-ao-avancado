use std::num::Saturating;

struct Compra {
    metodo_pagamento: MetodoPagamento,
}

struct Produto {
    nome: String,
    preco: f64,
}

enum MetodoPagamento {
    Pix(String),
    Cartao(String, u32),
    Boleto(String),
    Debito(String),
}

enum PagamentoErro {
    ValorInvalido(f64),
    ParcelasExcedidas(u32),
    CartaoRecusado(String),
    SaldoInsuficiente { tentou: f64, disponivel: f64 },
    CarrinhoVazio,
}

fn buscar_usuario(email: String) -> Result<String, String> {
    // simulacao de um db
    if email == "antonio@gmail.com" {
        Ok(String::from("Antonio"))
    } else if email == "maria@gmail.com" {
        Ok(String::from("Maria"))
    } else {
        Err(String::from("Usuario nao encontrado!"))
    }
}

fn processar_pagamento(metodo: MetodoPagamento, valor: f64) -> Result<(), PagamentoErro> {
    if valor <= 0.0 {
        return Err(PagamentoErro::ValorInvalido(valor));
    }
    match metodo {
        MetodoPagamento::Pix(chave) => {
            println!(
                "Gerando QR code para a chave {} no valor de: R$ {:.2}",
                chave, valor
            );
            Ok(())
        }
        MetodoPagamento::Cartao(numero, parcelas) => {
            if parcelas > 10 {
                return Err(PagamentoErro::ParcelasExcedidas(parcelas));
            }
            println!(
                "Cobrando R$ {:.2} no cartao {} em {}x",
                valor, numero, parcelas
            );
            Ok(())
        }
        MetodoPagamento::Boleto(codigo) => {
            println!("Gerando boleto no valor de R$ {:.2}", valor);
            println!("Codigo do boleto: {}", codigo);
            Ok(())
        }
        MetodoPagamento::Debito(numero) => {
            println!(
                "Passando R$ {:.2} no debito no carto com numero: {}",
                valor, numero
            );
            Ok(())
        }
    }
}

fn processar_pagamento_antiga(metodo: String, valor: f64) {
    if metodo == "pix" {
        println!("Gerando QR code no valor de: R$ {:.2}", valor);
    } else if metodo == "cartao" {
        println!("Cobrando R$ {:.2}", valor);
    } else if metodo == "boleto" {
        println!("Gerando boleto no valor de R$ {:.2}", valor);
    }
}

fn valida_valor(valor: f64) -> Result<f64, String> {
    if valor <= 0.0 {
        Err(String::from("Valor invalido"))
    } else {
        Ok(valor)
    }
}

fn finalizar_compra(
    email: String,
    carrinho: &[Produto],
    metodo: MetodoPagamento,
) -> Result<(), PagamentoErro> {
    if carrinho.is_empty() {
        return Err(PagamentoErro::CarrinhoVazio);
    }

    let valor = calcular_total(carrinho);

    if valor <= 0.00 {
        return Err(PagamentoErro::ValorInvalido(valor));
    }

    match metodo {
        MetodoPagamento::Pix(chave) => {
            println!(
                "Gerando QR code para a chave {} no valor de: R$ {:.2}",
                chave, valor
            );
        }
        MetodoPagamento::Cartao(numero, parcelas) => {
            if parcelas > 10 {
                return Err(PagamentoErro::ParcelasExcedidas(parcelas));
            }
            println!(
                "Cobrando R$ {:.2} no cartao {} em {}x",
                valor, numero, parcelas
            );
        }
        MetodoPagamento::Boleto(codigo) => {
            println!("Gerando boleto no valor de R$ {:.2}", valor);
            println!("Codigo do boleto: {}", codigo);
        }
        MetodoPagamento::Debito(numero) => {
            println!(
                "Passando R$ {:.2} no debito no carto com numero: {}",
                valor, numero
            );
        }
    }

    Ok(())
}

fn calcular_total(carrinho: &[Produto]) -> f64 {
    carrinho.iter().map(|p| p.preco).sum()
}

fn exibir_carrinho(carrinho: &[Produto]) {
    println!("=== Seu Carrinho ===");
    for (i, produto) in carrinho.iter().enumerate() {
        println!("{}. {} - R${:.2}", i + 1, produto.nome, produto.preco);
    }

    let total = calcular_total(carrinho);
    println!("Total: R${:.2}", total);
    println!("====================");
}

fn main() {
    let mut carrinho: Vec<Produto> = Vec::new();

    carrinho.push(Produto {
        nome: String::from("Macbook"),
        preco: 999.99,
    });

    carrinho.push(Produto {
        nome: String::from("Laptop Dell"),
        preco: 799.99,
    });

    carrinho.push(Produto {
        nome: String::from("Mouse Logitech"),
        preco: 29.99,
    });

    exibir_carrinho(&carrinho);

    let metodo = MetodoPagamento::Pix(String::from("richas@gmail.com"));

    match finalizar_compra(String::from("richas@gmail.com"), &carrinho, metodo) {
        Ok(()) => println!("Compra finalizada com sucesso!"),
        Err(erro) => match erro {
            PagamentoErro::CarrinhoVazio => {
                println!("Seu carrinho esta vazio!")
            }
            PagamentoErro::ValorInvalido(v) => {
                println!("Valor invalido: R${:.2}", v);
            }
            PagamentoErro::ParcelasExcedidas(p) => {
                println!("Parcelas excedidas: {}", p);
            }
            PagamentoErro::CartaoRecusado(motivo) => {
                println!("Seu cartao foi recusado: {}", motivo);
            }
            PagamentoErro::SaldoInsuficiente { tentou, disponivel } => {
                println!(
                    "Saldo insuficiente: tentou: {} - disponivel: {}",
                    tentou, disponivel
                )
            }
        },
    }
}
