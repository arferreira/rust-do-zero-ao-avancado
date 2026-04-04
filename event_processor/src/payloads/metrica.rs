use crate::traits::{Formatavel, Processavel};

pub struct MetricaPayload {
    pub nome: String,
    pub valor: f64,
}
impl Processavel for MetricaPayload {
    fn processar(&self) {
        println!("[METRICA][{}] {}", self.nome, self.valor);
    }
    fn nome_do_tipo(&self) -> &str {
        "metric"
    }
}
impl Formatavel for MetricaPayload {
    fn para_json(&self) -> String {
        format!(
            r#"{{"tipo": "metrica", "nome": "{}", "valor": "{}"}}"#,
            self.nome, self.valor
        )
    }

    fn para_texto(&self) -> String {
        format!("{} = {}", self.nome, self.valor)
    }
}
