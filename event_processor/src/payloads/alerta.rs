use crate::{
    Alertavel,
    traits::{Formatavel, Processavel},
};

// Traits mais comuns
// #[derive(Debug)]
// #[derive(Default)]
// #[derive(Clone)]
// #[derive(PartialEq)]
// #[derive(Eq)]
// #[derive(Hash)]

// Quando vc deve utilizar a derive ou implementar na mao?

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AlertaPayload {
    pub mensagem: String,
    pub severidade: u8,
}

impl Processavel for AlertaPayload {
    fn processar(&self) {
        let icone = if self.severidade >= 5 { "🔴" } else { "🟡" };
        println!("[ALERTA][{}] {}", icone, self.mensagem);
    }
    fn nome_do_tipo(&self) -> &str {
        "alert"
    }
}
impl Formatavel for AlertaPayload {
    fn para_json(&self) -> String {
        format!(
            r#"{{"tipo": "alerta", "mensagem": "{}", "severidade": "{}"}}"#,
            self.mensagem, self.severidade
        )
    }

    fn para_texto(&self) -> String {
        let icone = if self.severidade >= 5 { "🔴" } else { "🟡" };
        format!("[ALERTA][{}] {}", icone, self.mensagem)
    }
}

impl Alertavel for AlertaPayload {
    fn deve_alertar(&self) -> bool {
        self.severidade >= 5
    }
    fn nivel_alerta(&self) -> &str {
        match self.severidade {
            0..=3 => "BAIXO",
            4..=6 => "MEDIO",
            7..=9 => "ALTO",
            _ => "CRITICO",
        }
    }
}
