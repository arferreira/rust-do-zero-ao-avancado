struct Compra {
    metodo_pagamento: MetodoPagamento,
}

enum MetodoPagamento {
    Pix(String),
    Cartao(String, u32),
    Boleto(String),
    Debito(String),
}

fn processar_pagamento(metodo: MetodoPagamento, valor: f64) {
    match metodo {
        MetodoPagamento::Pix(chave) => {
            println!(
                "Gerando QR code para a chave {} no valor de: R$ {:.2}",
                chave, valor
            );
        }
        MetodoPagamento::Cartao(numero, parcelas) => {
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

fn main() {
    let pagamento = MetodoPagamento::Pix(String::from("pixdoantonio@gmail.com"));
    processar_pagamento(pagamento, 49.90);
    let cartao = MetodoPagamento::Cartao(String::from("1234 1234 1234 1234"), 10);
    processar_pagamento(cartao, 149.90);
    let debito = MetodoPagamento::Debito(String::from("1234 1234 1234 1234"));
    processar_pagamento(debito, 23.99);
    let boleto = MetodoPagamento::Boleto(String::from(
        "12323479.239872934798. 928374982374.237498237402",
    ));
    processar_pagamento(boleto, 59.99);
}
