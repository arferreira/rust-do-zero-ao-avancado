use event_processor::{
    AlertaPayload, Alertavel, Evento, LogPayload, MetricaPayload, Processavel, pipeline::Pipeline,
    pipeline_dinamico::PipelineDinamico,
};

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
    let log = LogPayload {
        mensagem: String::from("Teste"),
        nivel: String::from("INFO"),
    };
    let log2 = LogPayload::default();
    println!("{:?}", log2);

    println!("{:?}", log);

    let metrica = MetricaPayload {
        nome: String::from("cpu_usage"),
        valor: 80.0,
    };
    println!("{:?}", metrica);
    let metrica2 = metrica.clone();

    let metrica3 = MetricaPayload::default();
    println!("{:?}", metrica3);

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
    let mut pipeline1 = PipelineDinamico::new();

    for evento in eventos {
        pipeline1.adicionar(evento);
    }

    pipeline1.processar_todos();

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
