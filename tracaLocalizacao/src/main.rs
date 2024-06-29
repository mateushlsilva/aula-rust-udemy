use std::io;

fn invert(input: &str) -> String{
    return input.trim().chars().rev().collect::<String>();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler");
    let input_invert = invert(&input);
    println!("A palavra  {} invertida fica {}", input, input_invert);
}
