pub struct Loja {
    saldo: f64,
}

impl Loja {
    /// Cria uma nova loja com saldo zerado
    pub fn new() -> Self {
        Self { saldo: 0.0 }
    }

    /// Registra uma venda e incrementa o saldo da loja
    /// Retorna um erro se o valor da venda for menor que 0.0
    pub fn vender(&mut self, valor: f64) -> Result<f64, String> {
        if valor <= 0.0 {
            return Err(String::from("O valor da venda deve ser maior do que zero!"));
        }

        self.saldo += valor;
        Ok(valor)
    }

    /// Retorna o saldo atual da loja
    pub fn saldo(&self) -> f64 {
        self.saldo
    }

    // Retira um valor do saldo da loja
    // Entra em panico se o saldo ficar negativo
    pub fn retirada(&mut self, valor: f64) {
        if valor > self.saldo {
            panic!("Saldo insuficiente: Tentou retirar valor maior que o saldo!");
        }
        self.saldo -= valor;
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn loja_comeca_com_saldo_zerado() {
        let loja = Loja::new();
        assert_eq!(loja.saldo(), 0.0);
    }

    #[test]
    fn deve_falhar_venda_com_valor_negativo() {
        let mut loja = Loja::new();
        let result = loja.vender(-49.99);
        assert!(result.is_err());
        assert_eq!(loja.saldo(), 0.0);
    }

    // #[test] -> False positivo
    // fn venda_funciona() {
    //     let mut loja = Loja::new();
    //     let _ = loja.vender(100.0).unwrap();
    //     assert!(loja.saldo() > 0.0);
    // }

    #[test]
    #[should_panic(expected = "Saldo insuficiente")]
    fn deve_entrar_em_panico_se_deixar_o_saldo_negativo() {
        let mut loja = Loja::new();
        let _ = loja.vender(100.0).unwrap();
        loja.retirada(200.0);
    }

    #[test]
    fn venda_incrementa_o_saldo() {
        let mut loja = Loja::new();
        let _ = loja.vender(100.0).unwrap();
        assert_eq!(loja.saldo(), 100.0);
        assert_ne!(loja.saldo(), 0.0);
    }

    #[test]
    fn multiplas_vendas_acumulam_saldo() {
        let mut loja = Loja::new();
        let _ = loja.vender(100.0).unwrap();
        let _ = loja.vender(100.0).unwrap();
        let _ = loja.vender(100.0).unwrap();
        assert_eq!(loja.saldo(), 300.0);
    }
}
