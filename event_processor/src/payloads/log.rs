use crate::traits::{Formatavel, Processavel};
use std::fmt;

#[derive(Debug, Clone, Default)]
pub struct LogPayload {
    pub mensagem: String,
    pub nivel: String,
}

impl Processavel for LogPayload {
    fn processar(&self) {
        println!("[LOG][{}] {}", self.nivel, self.mensagem);
    }

    fn nome_do_tipo(&self) -> &str {
        "log"
    }

    fn resumo(&self) -> String {
        format!("[{}] {}", self.nivel, self.mensagem)
    }
}

impl Formatavel for LogPayload {
    fn para_json(&self) -> String {
        format!(
            r#"{{"tipo": "log", "nivel": "{}", "mensagem": "{}"}}"#,
            self.nivel, self.mensagem
        )
    }

    fn para_texto(&self) -> String {
        format!("[{}] {}", self.nivel, self.mensagem)
    }
}

impl std::fmt::Display for LogPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.nivel, self.mensagem)
    }
}

// Orphan Rule
// Newtype Pattern
pub struct Logs(pub Vec<String>);

impl std::fmt::Display for Logs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in &self.0 {
            write!(f, "{}", item)?;
        }
        Ok(())
    }
}
