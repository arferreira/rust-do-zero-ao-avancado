use event_processor::{AlertaPayload, Alertavel, Evento, LogPayload, pipeline::Pipeline};

fn verificar_alertas<T: Alertavel>(eventos: &[Evento<T>]) {
    for evento in eventos {
        let payload = evento.payload();
        if payload.deve_alertar() {
            payload.disparar_alerta();
        } else {
            println!("{} - ok: {}", payload.nome_do_tipo(), payload.para_texto());
        }
    }
}

fn main() {
    let alertas = vec![
        Evento::new(
            289371928,
            AlertaPayload {
                mensagem: String::from("Disco quase cheio!"),
                severidade: 9,
            },
        ),
        Evento::new(
            23894729387,
            AlertaPayload {
                mensagem: String::from("CPU levemente alta"),
                severidade: 3,
            },
        ),
    ];

    verificar_alertas(&alertas);

    let mut pipeline: Pipeline<LogPayload> = Pipeline::new();
    pipeline.adicionar(Evento::new(
        170998530498,
        LogPayload {
            mensagem: String::from("Rapina foi iniciado"),
            nivel: String::from("INFO"),
        },
    ));
    pipeline.adicionar(Evento::new(
        170998530498,
        LogPayload {
            mensagem: String::from("Request recebida"),
            nivel: String::from("DEBUG"),
        },
    ));
    pipeline.processar_todos();

    println!();

    let alerta = Evento::new(
        13284208,
        AlertaPayload {
            mensagem: String::from("Disco do DB esta cheio"),
            severidade: 5,
        },
    );
    alerta.processar_evento();
}
