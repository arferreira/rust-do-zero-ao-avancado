use crate::traits::Processavel;

pub struct PipelineDinamico {
    eventos: Vec<Box<dyn Processavel>>,
}

impl PipelineDinamico {
    pub fn new() -> Self {
        Self {
            eventos: Vec::new(),
        }
    }
    pub fn adicionar(&mut self, evento: Box<dyn Processavel>) {
        self.eventos.push(evento)
    }

    pub fn total(&self) -> usize {
        self.eventos.len()
    }

    pub fn processar_todos(&self) {
        println!("====== Processando {} eventos ====", self.total());
        for evento in &self.eventos {
            println!("[{}] {}", evento.nome_do_tipo(), evento.resumo());
            evento.processar();
        }
        println!("==== FIM ====");
    }
}
