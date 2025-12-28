// Controle de repetição ou Laços de repeticao
// - Loops
// - For
// - While

fn main() {
    // Loops
    let s = 1;

    loop {
        println!("Este é um laço de repetição loop");
        if s == 10 {
            break;
        }
    }

    'fora: loop {
        'interno: loop {
            println!("Este é mais um laço de repetição usando loop encadeado");
            if s == 10 {
                break 'interno;
            }
        }
        if s == 100 {
            break 'fora;
        }
    }

    // atribuindo a variavel
    let x = loop {
        if s > 0 {
            break 5;
        }
    };

    // For
    let v = vec![1, 2, 3, 4, 5, 6];
    for item in v {
        println!("{item}");
    }

    // while
    let mut numero = 0;
    while numero < 10 {
        // numero = numero + 1;
        numero += 1;
    }
}

