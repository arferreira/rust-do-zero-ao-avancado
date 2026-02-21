use std::num::Saturating;

struct Compra {
    metodo_pagamento: MetodoPagamento,
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

fn finalizar_compra(email: String, metodo: MetodoPagamento, valor: f64) -> Result<(), String> {
    let nome = buscar_usuario(email)?;
    let valor = valida_valor(valor)?;

    println!("Processando compra de {} no valor de R${:.2}", nome, valor);
    Ok(())
}

fn main() {
    let email = String::from("antonio@gmail.com");

    if let Ok(nome) = buscar_usuario(email) {
        println!("Ola, {}! Processando seu pagamento....", nome);

        let pagamento = MetodoPagamento::Cartao(String::from("4242424242424242"), 10);
        match processar_pagamento(pagamento, 100.0) {
            Ok(()) => println!("Pagamento realizado com sucesso!"),
            Err(erro) => match erro {
                PagamentoErro::ValorInvalido(v) => {
                    println!("Valor: R$ {} nao e valido", v)
                }
                PagamentoErro::ParcelasExcedidas(p) => {
                    println!("{}x nao e permitido", p)
                }
                PagamentoErro::CartaoRecusado(motivo) => {
                    println!("Cartao recusado: {}", motivo);
                    println!("Tente outro metodo de pagamento");
                }
                PagamentoErro::SaldoInsuficiente { tentou, disponivel } => {
                    println!(
                        "Saldo insuficiente. Tentou R${:.2}, porem disponivel R${:.2}",
                        tentou, disponivel
                    )
                }
            },
        }
    } else {
        println!("Usuario nao encontrado. Crie sua conta primeiro.")
    }
}
