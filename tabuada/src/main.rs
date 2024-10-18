use std::io;

fn convert_to_int(input: &str) -> i32{
    let x = input.trim().parse::<i32>().unwrap();
    return x
}

fn tabuada_1_to_10(numero: i32){
    println!("A tabuada do número {} é: ", numero);
    for n in 1..11{
        println!("{} x {} = {}",numero, n, numero * n);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler");
    let number_i32: i32 = convert_to_int(&input);
    tabuada_1_to_10(number_i32);
}
