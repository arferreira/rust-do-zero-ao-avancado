use crate::LogPayload;
use crate::traits::Processavel;
use std::fmt::Display;

pub struct Evento<T> {
    pub timestamp: u64,
    pub payload: T,
}
pub enum ProcessamentoResultado<T> {
    Sucesso(T),
    Descartado,
    Erro(String),
}

// Sao os metodos disponiveis para QUALQUER T
impl<T> Evento<T> {
    pub fn new(timestamp: u64, payload: T) -> Self {
        Self { timestamp, payload }
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn payload(&self) -> &T {
        &self.payload
    }
}

impl Evento<String> {
    pub fn payload_em_maiusculo(&self) -> String {
        self.payload.to_uppercase()
    }
}

impl Evento<f64> {
    pub fn payload_arredondado(&self) -> f64 {
        (self.payload * 100.0).round() / 100.0
    }
}

impl<T: Processavel> Evento<T> {
    pub fn processar_evento(&self) {
        println!("---- Evento em {} ----", self.timestamp);
        self.payload.processar();
        println!("Resumo: {}", self.payload.resumo());
        println!();
    }
}
fn criar_evento_padrao() -> Evento<impl Processavel> {
    Evento::new(
        0,
        LogPayload {
            mensagem: String::from("Evento padrao"),
            nivel: String::from("INFO"),
        },
    )
}
fn processar_e_exibir<T>(payload: &T)
where
    T: Processavel + Display + Clone + PartialEq,
{
    println!("processar_e_exibiro: {}", payload);
    payload.processar();
    payload.resumo();
}

pub fn exibir<T: Display>(evento: &Evento<T>) {
    println!("[{}] {}", evento.timestamp, evento.payload());
}

pub fn processar_evento<T>(evento: &Evento<T>) -> ProcessamentoResultado<u64> {
    if evento.timestamp() == 0 {
        return ProcessamentoResultado::Erro(String::from("Timestamp invalido"));
    }

    ProcessamentoResultado::Sucesso(evento.timestamp())
}
