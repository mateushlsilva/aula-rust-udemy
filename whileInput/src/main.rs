use std::io;

fn covert_to_int(input: &str) -> i32{
    let x = input.trim().parse::<i32>().unwrap();
    return  x;
}

fn main() {
    let mut soma = 0;
    let mut valor = String::new();
    io::stdin().read_line(&mut valor).expect("Erro ao ler");
    let mut valor_int : i32 = covert_to_int(&valor);
    while valor_int != 0 {
        let mut r = valor_int % 10;
        soma = soma + r;
        valor_int = valor_int / 10;
    }
    println!("O valor da soma dos digitos {}", soma)

}
