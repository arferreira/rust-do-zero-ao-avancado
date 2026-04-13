use event_processor::{
    AlertaPayload, Evento, LogPayload, MetricaPayload, Processavel, evento, pipeline,
    pipeline_dinamico::PipelineDinamico,
};
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

#[test]
fn pipeline_dinamico_aceita_tipos_misturados() {
    let mut pipeline = PipelineDinamico::new();
    let eventos: Vec<Box<dyn Processavel>> = vec![
        Box::new(LogPayload {
            mensagem: String::from("Rapina foi iniciado"),
            nivel: String::from("INFO"),
        }),
        Box::new(MetricaPayload {
            nome: String::from("cpu"),
            valor: 73.3,
        }),
        Box::new(AlertaPayload {
            mensagem: String::from("Disco cheio"),
            severidade: 5,
        }),
    ];

    for evento in eventos {
        pipeline.adicionar(evento);
    }
    assert_eq!(pipeline.total(), 3);
}

#[test]
fn debug_funciona() {
    let log = LogPayload {
        mensagem: String::from("test"),
        nivel: String::from("INFO"),
    };
    let debug = format!("{:?}", log);
    assert!(debug.contains("test"));
    assert!(debug.contains("INFO"));
}
