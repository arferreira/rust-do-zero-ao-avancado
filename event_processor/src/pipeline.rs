use crate::Evento;
use crate::traits::Processavel;

pub struct Pipeline<T: Processavel> {
    eventos: Vec<Evento<T>>,
}

impl<T: Processavel> Pipeline<T> {
    pub fn new() -> Self {
        Self {
            eventos: Vec::new(),
        }
    }

    pub fn adicionar(&mut self, evento: Evento<T>) {
        self.eventos.push(evento);
    }

    pub fn processar_todos(&self) {
        for evento in &self.eventos {
            evento.processar_evento();
        }
    }
}
