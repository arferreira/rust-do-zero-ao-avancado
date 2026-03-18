pub enum ProcessamentoResultado<T> {
    Sucesso(T),
    Descartado,
    Erro(String),
}

pub struct Evento<T> {
    timestamp: u64,
    payload: T,
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

pub fn exibir_texto<T>(evento: &Evento<T>) {
    println!("[{}]", evento.timestamp);
}

pub fn processar_evento<T>(evento: &Evento<T>) -> ProcessamentoResultado<u64> {
    if evento.timestamp() == 0 {
        return ProcessamentoResultado::Erro(String::from("Timestamp invalido"));
    }

    ProcessamentoResultado::Sucesso(evento.timestamp())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evento_string_criado_corretamente() {
        let evento = Evento::new(100, String::from("teste"));
        assert_eq!(evento.timestamp(), 100);
        assert_eq!(evento.payload(), "teste");
    }

    #[test]
    fn evento_f64_criado_corretamente() {
        let evento = Evento::new(200, 42.5);
        assert_eq!(evento.timestamp(), 200);
        assert_eq!(*evento.payload(), 42.5);
    }

    #[test]
    fn evento_bool_criado_corretamente() {
        let evento = Evento::new(300, true);
        assert_eq!(evento.timestamp(), 300);
        assert_eq!(*evento.payload(), true);
    }

    #[test]
    fn payload_maiusculo_so_funciona_com_string() {
        let evento = Evento::new(100, String::from("hello"));
        assert_eq!(evento.payload_em_maiusculo(), "HELLO");
    }

    #[test]
    fn payload_arredondado_so_funciona_com_f64() {
        let evento = Evento::new(100, 42.567);
        assert_eq!(evento.payload_arredondado(), 42.57);
    }
}
