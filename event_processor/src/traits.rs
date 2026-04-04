pub trait Processavel {
    fn processar(&self);
    fn nome_do_tipo(&self) -> &str;
    fn resumo(&self) -> String {
        format!("[{}] processado", self.nome_do_tipo())
    }
}

pub trait Formatavel {
    fn para_json(&self) -> String;
    fn para_texto(&self) -> String;
}

pub trait Alertavel: Processavel + Formatavel {
    fn deve_alertar(&self) -> bool;
    fn nivel_alerta(&self) -> &str;

    fn disparar_alerta(&self) {
        if self.deve_alertar() {
            println!("ALERTA!!! [{}]: {}", self.nivel_alerta(), self.para_texto());
            println!(" JSON: {}", self.para_json());
            println!(" Tipo: {}", self.nome_do_tipo());
        }
    }
}
