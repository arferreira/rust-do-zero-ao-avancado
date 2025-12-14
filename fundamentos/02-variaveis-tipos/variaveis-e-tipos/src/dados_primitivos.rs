/*
*  Tipos de dados basicos
*  - Integers
*  - Floats
*  - Chars
*  - Boolean
*/

pub fn data_types() {
    // Unsigned Integers
    let unsigned_num: u8 = 5; // u8, u16, u32, u64, u128

    // Signed Integers
    let signed_num: i8 = 10; // i8, i16, i32, i64, i128, i32

    // Floats
    let floating_num: f32 = 14.0; // f32, f64 - f64 padrão

    // tipo de alocação para inteiros baseado na arquitetura
    let arch1: usize = 5;
    let arch2: isize = 8;

    // Chars
    let caractere: char = 'a';

    // Boolean
    let b: bool = false; // true

    // Alias
    type Idade = u8;
    let idade_antonio: Idade = 18;

    // Conversao de tipos de dados
    let a: i32 = 10;
    let b = a as f64;
}
