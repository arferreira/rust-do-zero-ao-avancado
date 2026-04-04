use event_processor::{Evento, LogPayload};
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

#[test]
fn evento_com_payload_processavel() {
    let evento = Evento::new(
        178923828349,
        LogPayload {
            mensagem: String::from("Rapina foi iniciado"),
            nivel: String::from("INFO"),
        },
    );
    evento.processar_evento();
}
