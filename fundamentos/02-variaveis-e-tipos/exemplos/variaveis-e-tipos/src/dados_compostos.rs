/*
* Tipos de dados compostos
* - &str e String
* - Arrays
* - Vetores
* - Tuplas
* - Tupla vazia
*
* Exercicios:
* - Ex1: https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=6db26333288f93021ec01b3358d4075c
* - Ex2: https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=e789fad69c6f9768fb3fe4309cbf4296
* - EX3: https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=b8b4c79ad0a352f294fb96c5662545e2
*/

pub fn compound_data_types() {
    // Strings (&str e String)
    let string_de_tamanho_fixo: &str = "antonio";
    let mut string_de_tamanho_flexivel: String = String::from("joão");
    string_de_tamanho_flexivel.push('s');

    // Arrays
    let array1: [i32; 5] = [1, 2, 3, 4, 5];
    let numero = array1[1] as u8;
    println!("{:?}", numero);

    let array2 = [0; 10];

    //  Vetores
    let vetor1: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    let numero = vetor1[3]; // 4

    // Tuplas
    let minhas_informacoes: (&'static str, &'static str, &'static str, i8) =
        ("Nome", "Antonio", "Idade", 18);
    let idade = minhas_informacoes.3;

    // desestruturacao de Tuplas
    let (nome, nome_valor, idade, idade_valor) = minhas_informacoes;

    let unit: () = ();
}

