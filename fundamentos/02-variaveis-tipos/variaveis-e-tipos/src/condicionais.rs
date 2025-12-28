// if, else, else if

enum Direcao {
    Norte,
    Sul,
    Leste,
    Oeste,
}

pub fn condicionais() {
    let direcao = Direcao::Norte;

    match direcao {
        Direcao::Norte => println!("Estamos indo para o Norte"),
        Direcao::Sul => println!("Estamos indo para o Sul"),
        Direcao::Leste => println!("Estamos indo para o Leste"),
        Direcao::Oeste => println!("Estamos indo para o Oeste"),
        _ => println!("Não sei para onde estamos indo"),
    }

    let idade = 233;

    if idade > 5 && idade < 18 {
        println!("Você está muito jovem para aprender Rust!");
    } else if idade > 18 && idade <= 100 {
        println!("Você é ainda mais jovem para aprender Rust!");
    } else {
        println!("Infelizmente, eu ainda acho você jovem para iniciar em Rust!");
    }

    // pattern matching
    let idade = 28;
    let mut message = "";

    match idade {
        1..=18 => {
            let x = 10;
            message = "Você ainda é menor de idade"
        }
        19..=100 => message = "Você é maior de idade",
        _ => message = "Não conheço a sua idade",
    }

    let message = match idade {
        1..=18 => "Você é menor de idade",
        19..=100 => "Você é maior de idade",
        _ => "Não conheço a sua idade",
    };

    println!("Message é {message}");

    let x = 3;
    match x {
        1 => println!("Um"),
        2 => println!("Dois"),
        3 => println!("Tres"),
        _ => println!("Qualquer outro número"),
    }

    let mut message = "";
    match idade {
        1..=18 => message = "menor",
        19..=100 => message = "maior",
        _ => message = "nao conheço",
    }
}
