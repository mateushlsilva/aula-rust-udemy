use std::io;

fn convert_to_int(input: &str) -> i32{
    let x = input.trim().parse::<i32>().unwrap();
    return x
}

fn fatorial(n: i32) -> i32{
    if n <= 1{
        return 1
    }
    return n * fatorial(n - 1)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler");
    let number_i32: i32 = convert_to_int(&input);
    let fat = fatorial(number_i32);
    println!("O fatorial de {} é {}", number_i32, fat);
}
